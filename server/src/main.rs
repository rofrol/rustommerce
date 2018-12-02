// http://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust
// https://users.rust-lang.org/t/turning-off-compiler-warning-messages/4975/2
#![allow(non_snake_case)]

// mod api;
// mod cors;
// mod files;

extern crate actix;
extern crate actix_web;
extern crate env_logger;

extern crate postgres;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate dotenv;
use dotenv::dotenv;
use std::env;

use actix_web::{middleware, server, App, FromRequest, HttpRequest, Path};

#[derive(Serialize)]
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

#[derive(Deserialize)]
struct TemplateParams {
    ssr: bool,
}

fn template(req: &HttpRequest) -> String {
    let params = actix_web::Path::<TemplateParams>::extract(req).unwrap();
    format!("ssr {}", params.ssr)
    // let s2 = getStr();
    // println!("s2 {}", s2);
    // let s: String = if ssr { s2 } else { "".to_owned() };
    // let context = TemplateContext {
    //     parent: "index".to_owned(),
    //     name: "Roman".to_owned(),
    //     content: s.to_owned(),
    //     items: vec!["One", "Two", "Three"]
    //         .iter()
    //         .map(|s| s.to_string())
    //         .collect(),
    // };
    // CORS::any(Template::render("template", &context))
}

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
extern crate time;

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
            .write(str1.as_bytes())
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
            .resource("/template/{ssr}", |r| r.f(template))
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
