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
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::{JSON, Value};

extern crate dotenv;

#[error(404)]
fn not_found() -> JSON<Value> {
    JSON(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    println!("{:?}", dotenv!("data_server"));
    rocket::ignite()
        .mount("/", routes![files::index, files::redirect_to_index, files::js, files::styles, files::styles_with_query, files::images])
        .mount("/api",
               routes![api::user_information, api::data_sets, api::data_set, api::data_set_category, api::data_sets_categories, api::data_sets_new])
        .catch(errors![not_found])
        .launch();
}
