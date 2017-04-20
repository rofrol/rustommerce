#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![plugin(dotenv_macros)]

// http://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust
// https://users.rust-lang.org/t/turning-off-compiler-warning-messages/4975/2
#![allow(non_snake_case)]

extern crate rocket;

mod files;
mod api;

extern crate postgres;
extern crate serde_json;
// for !json macro
// #[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
use rocket_contrib::Template;
use rocket::Request;

#[derive(Serialize)]
struct TemplateContext {
    parent: String,
    name: String,
    content: String,
    items: Vec<String>,
}

use std::process::Command;
use std::path::Path;
use std::fs::File;
use std::io::Read;

#[get("/template/<ssr>")]
fn template(ssr: bool) -> Template {
    println!("ssr {}", ssr);
    let s: String = if ssr { getStr() } else { "".to_owned() };
    // let s: String = getStr();
    // s.dupa;
    let context = TemplateContext {
        parent: "index".to_owned(),
        name: "Roman".to_owned(),
        content: s.to_owned(),
        items: vec!["One", "Two", "Three"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };
    Template::render("template", &context)
}

fn getStr() -> String {
    let _ = Command::new("node ")
        .current_dir(&Path::new("../client"))
        .arg("./node_modules/elm-static-html/index.js")
        .arg("-c")
        .arg("elm-static-html.json")
        .output()
        .expect("elm-static-html command failed to start");

    let mut file = File::open("../client/dist/body-static.html").unwrap();

    let mut contents: Vec<u8> = Vec::new();
    // Returns amount of bytes read and append the result to the buffer
    let result = file.read_to_end(&mut contents).unwrap();
    println!("Read {} bytes", result);
    let s = String::from_utf8_lossy(&*contents);
    s.into_owned()
}

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

fn main() {
    println!("{:?}", dotenv!("data_server"));
    rocket::ignite()
        .mount("/",
               routes![template,
                       files::favicon,
                       files::index,
                       files::redirect_to_index,
                       files::js,
                       files::styles,
                       files::styles_with_query,
                       files::images])
        .mount("/api",
               routes![api::user_information,
                       api::data_sets,
                       api::data_set,
                       api::data_set_category,
                       api::data_sets_categories,
                       api::data_sets_new])
        .catch(errors![not_found])
        .launch();
}
