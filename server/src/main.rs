#![recursion_limit = "256"]
// http://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust
// https://users.rust-lang.org/t/turning-off-compiler-warning-messages/4975/2
#![allow(non_snake_case)]

mod api;
// mod cors;
// mod files;

use serde_derive::Serialize;

use dotenv::dotenv;
use std::env;

use actix_web::{http::Method, middleware, server, App, FromRequest, HttpRequest, HttpResponse};

use futures::future::{result, FutureResult};

use typed_html::elements::FlowContent;
use typed_html::types::Metadata;
use typed_html::{dom::DOMTree, html, text, OutputType};

#[derive(Serialize, Debug)]
struct TemplateContext {
    parent: String,
    name: String,
    content: String,
    items: Vec<String>,
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world"
}

use std::path;
use std::process::Command;

fn template(req: &HttpRequest) -> FutureResult<HttpResponse, actix_web::error::Error> {
    let ssr = *actix_web::Path::<bool>::extract(req).expect("Path extract failed");
    let s2 = getStr();
    let s: String = if ssr { s2.to_owned() } else { "".to_owned() };
    let context = TemplateContext {
        parent: "index".to_owned(),
        name: "Roman".to_owned(),
        content: s.to_owned(),
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

    result(Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(doc_str)))
}

fn doc<T: OutputType + 'static>(
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
            time = time::get_time().sec
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
        .skip_while(|x| x.find("id=\"content\"").is_none())
        .collect()
}

// #[error(404)]
// fn not_found(req: &Request) -> CORS<Template> {
//     let mut map = std::collections::HashMap::new();
//     map.insert("path", req.uri().as_str());
//     CORS::any(Template::render("error/404", &map))
// }

fn main() {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("hello-world");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.f(index))
            .resource("/template/{ssr}", |r| r.method(Method::GET).a(template))
            .resource("/userInformation", |r| {
                r.method(Method::GET).a(api::user_information)
            })
            .resource("/dataSets", |r| r.method(Method::GET).a(api::data_sets))
            .resource("/dataSets/{url}", |r| {
                r.method(Method::GET).a(api::data_set)
            })
            .resource("/dataSetsCategories/{url}", |r| {
                r.method(Method::GET).a(api::data_set_category)
            })
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Starting http server: 127.0.0.1:8080");

    let _ = sys.run();

    // println!("{:?}", env::var("data_server").unwrap());
    // rocket::ignite()
    //     .mount(
    //         "/",
    //         routes![
    //             template,
    //             files::favicon,
    //             files::index,
    //             files::redirect_to_index,
    //             files::js,
    //             files::styles,
    //             files::styles_with_query,
    //             files::images
    //         ],
    //     )
    //     .mount(
    //         "/api",
    //         routes![
    //             api::user_information,
    //             api::data_sets,
    //             api::data_set,
    //             api::data_set_category,
    //             api::data_sets_categories,
    //             api::data_sets_new
    //         ],
    //     )
    //     .catch(errors![not_found])
    //     .launch();
}
