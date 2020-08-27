#![recursion_limit = "256"]
// http://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust
// https://users.rust-lang.org/t/turning-off-compiler-warning-messages/4975/2
#![allow(non_snake_case)]

mod api;
mod files;

use serde_derive::Serialize;

use dotenv::dotenv;
use std::env;

use actix_web::{guard, middleware, web, App, Error as ActixError, HttpResponse, HttpServer};

use typed_html::elements::FlowContent;
use typed_html::types::Metadata;
use typed_html::{dom::DOMTree, html, text, OutputType};

use tinytemplate::TinyTemplate;

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

static TEMPLATE: &str = "Hello {name}!";

#[derive(Serialize)]
struct Context {
    name: String,
}

#[derive(Serialize, Debug)]
struct TemplateContext {
    parent: String,
    name: String,
    content: String,
    items: Vec<String>,
}

use std::path;
use std::process::Command;

async fn template(ssr: web::Path<bool>) -> Result<HttpResponse, ActixError> {
    let s2 = getStr();
    let s: String = if *ssr { s2.to_owned() } else { "".to_owned() };
    let context = TemplateContext {
        parent: "index".to_owned(),
        name: "Roman".to_owned(),
        content: s,
        items: vec!["One", "Two", "Three"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };

    let doc: DOMTree<String> = doc(
        html!(
            <div>
                <h1>"Hello Kitty"</h1>
                <p class="official">
                    "She is not a cat. She is a human girl."
                </p>
                { (0..3).map(|_| html!(
                    <p class="emphasis">
                        "Her name is Kitty White."
                    </p>
                )) }
                <p class="citation-needed">
                    "We still don't know how she eats."
                </p>
                <h1>"Hello Kitty"</h1>
                "DYNAMIC_CONTENT"
            </div>
        ),
        context,
    );
    let mut doc_str = "<!doctype html>".to_owned() + &doc.to_string();

    // https://users.rust-lang.org/t/how-to-create-a-macro-from-dynamic-content/5079
    doc_str = doc_str.replacen("DYNAMIC_CONTENT", &s2, 1);

    Ok(HttpResponse::Ok().content_type("text/html").body(doc_str))
}

fn doc<T: OutputType + 'static + Send>(
    tree: Box<dyn FlowContent<T>>,
    context: TemplateContext,
) -> DOMTree<T> {
    let TemplateContext { name, items, .. } = context;
    html!(
        <html>
            <head>
                <title>"Hello Kitty"</title>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <link rel="icon" href="/favicon.ico?v=1" />
                <link href="/styles/normalize.css" rel="stylesheet" type="text/css" media="all" />
                <link href="/styles/style.css" rel="stylesheet" type="text/css" media="all" />
                <base href="/"></base>
                <meta name=Metadata::Author content="Not Sanrio Co., Ltd" />
            </head>
            <body>
                 <h1>{text!("{}", name)}</h1>
                 <h3>"Here are your items:"</h3>
                 <ul>
                    {
                     items.iter().map(|item| html!(
                           <li>{text!("{}", item)}</li>
                     ))
                    }
                 </ul>
                 { tree }
                <p>"Try going to "<a href="/hello/YourName">"/hello/YourName"</a></p>
            </body>
        </html>
    )
}

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn getStr() -> String {
    fs::copy(
        "../client/src/elm/Main.elm",
        "../client/src/elm/Main.elm.bak",
    )
    .expect("file not copied");
    let stdout: Vec<u8>;
    {
        let mut main_file = OpenOptions::new()
            .append(true)
            .open("../client/src/elm/Main.elm")
            .expect("file not opened");

        let str1 = format!(
            r#"
            view : Html Msg
            view =
                viewWithModel <|
                    Model [] HomeRoute <|
                        Flags "" {time} ""
            "#,
            time = time::OffsetDateTime::unix_epoch().second()
        );

        main_file
            .write_all(str1.as_bytes())
            .expect("file content not saved");

        stdout = Command::new("node")
            .current_dir(&path::Path::new("../client"))
            .arg("./node_modules/elm-static-html/index.js")
            .arg("-f")
            .arg("src/elm/Main.elm")
            .output()
            .expect("elm-static-html command failed to start")
            .stdout;
    }
    fs::rename(
        "../client/src/elm/Main.elm.bak",
        "../client/src/elm/Main.elm",
    )
    .expect("file not renamed");

    String::from_utf8_lossy(&*stdout)
        .lines()
        .skip_while(|x| x.find(r#"id="content""#).is_none())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let endpoint = format!("127.0.0.1:{}", env::var("SERVER_PORT")?);

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        &env::var("DBUSER")?,
        &env::var("DBPASS")?,
        &env::var("DBHOST")?,
        &env::var("DBPORT")?,
        &env::var("DBNAME")?,
    );
    let pg_config: tokio_postgres::Config = connection_string.parse()?;
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::new(mgr, 16);

    let mut tt = TinyTemplate::new();
    tt.add_template("hello", TEMPLATE)?;

    let context = Context {
        name: "TinyTemplate".to_string(),
    };

    let rendered = tt.render("hello", &context)?;
    println!("{}", rendered);

    let sys = actix_rt::System::new("app");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/template/{ssr}").route(web::get().to(template)))
            .service(web::resource("/userInformation").route(web::get().to(api::user_information)))
            .service(web::resource("/dataSets").route(web::get().to(api::data_sets)))
            .service(web::resource("/dataSets/{url}").route(web::get().to(api::data_set)))
            .service(
                web::resource("/dataSetsCategories")
                    .route(web::get().to(api::data_sets_categories)),
            )
            .service(
                web::resource("/dataSetsCategories/{url}")
                    .route(web::get().to(api::data_set_category)),
            )
            .service(web::resource("/favicon").route(web::get().to(files::favicon)))
            .service(web::resource("/styles/{file:.*}").route(web::get().to(files::styles)))
            .service(web::resource("/js/{file:.*}").route(web::get().to(files::js)))
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(files::index))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind(&endpoint)?
    .run();
    // When main returns `Result<(), Box<dyn std::error::Error + 'static + Send + Sync>> instead of
    // `io::Result<()>`, there is error and `into()` is needed.
    // expected struct `std::boxed::Box`, found struct `std::io::Error`
    // https://users.rust-lang.org/t/boxing-errors-in-result-throws-type-mismatch/36692/2
    //.map_err(|e| e.into())

    println!("Server running at {:?}", endpoint);
    let _ = sys.run();

    Ok(())
}
