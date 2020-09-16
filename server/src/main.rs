//#![recursion_limit = "256"]
// http://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust
// https://users.rust-lang.org/t/turning-off-compiler-warning-messages/4975/2

mod api;
mod files;

use serde_derive::Serialize;

use std::env;

use actix_web::{guard, middleware, web, App, Error as ActixError, HttpResponse, HttpServer};

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

#[macro_use]
extern crate horrorshow;
use horrorshow::helper::doctype;
use horrorshow::{RenderBox, Template};

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

async fn template(ssr: web::Path<bool>) -> Result<HttpResponse, ActixError> {
    let s: String = if *ssr {
        "true".to_owned()
    } else {
        "false".to_owned()
    };

    let posts = vec![
        Post {
            title: String::from("First Post"),
            tags: vec![String::from("first post")],
            body: String::from("My Test Post"),
        },
        Post {
            title: String::from("Second Post"),
            tags: vec![],
            body: String::from("My Second Test Post"),
        },
    ];

    let doc_str = render("my blog", posts.into_iter(), &s);

    Ok(HttpResponse::Ok().content_type("text/html").body(doc_str))
}

fn render_post(post: Post) -> Box<dyn RenderBox> {
    let Post { title, body, tags } = post;
    box_html! {
        article {
            header(class="post-header") {
                h1 : title;
                ul {
                    @ for tag in tags {
                        li : tag
                    }
                }
            }
            section(class="post-body") : body;
        }
    }
}

fn render<I: Iterator<Item = Post>>(title: &str, posts: I, elm_string: &str) -> String {
    (html! {
        : doctype::HTML;
        html {
            head {
                title : title
            }
            body {
                main {
                    header { h1 : title }
                    section(id="posts") {
                        @ for post in posts {
                            : render_post(post)
                        }
                    }
                    div: elm_string
                }
            }
        }
    })
    .into_string()
    .unwrap()
}

struct Post {
    title: String,
    tags: Vec<String>,
    body: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Error: NotPresent
    // Chaning to specific file, bc `strace ./target/debug/rustommerce` shown,
    // that dotenv searches up for .env till it reaches /
    // https://galenguyer.com/blog/2020/05/19/docker-rust-notpresent
    let my_path = env::current_dir().map(|a| a.join("../.env"))?;
    dotenv::from_path(my_path.as_path())?;

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
    // Idea for parse from
    // https://github.com/OneSignal/L3-37/blob/2031639d1f3a7aa00a5a741b87b464e19d995d21/l337-postgres/src/lib.rs#L217
    let pg_config: tokio_postgres::Config = connection_string.parse()?;
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::new(mgr, 16);

    let sys = actix_rt::System::new("app");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/")
                    .service(
                        web::resource("/products")
                            .route(web::get().to(api::handlers::get_products)),
                    )
                    .service(
                        web::resource("/userInformation")
                            .route(web::get().to(api::user_information)),
                    )
                    .service(web::resource("/dataSets").route(web::get().to(api::data_sets)))
                    .service(web::resource("/dataSets/{url}").route(web::get().to(api::data_set)))
                    .service(
                        web::resource("/dataSetsCategories")
                            .route(web::get().to(api::data_sets_categories)),
                    )
                    .service(
                        web::resource("/dataSetsCategories/{url}")
                            .route(web::get().to(api::data_set_category)),
                    ),
            )
            .service(web::resource("/template/{ssr}").route(web::get().to(template)))
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
    // If you do not return `Ok(())` but just `.run()`, this will happend:
    // When main returns `Result<(), Box<dyn std::error::Error + 'static + Send + Sync>> instead of
    // `io::Result<()>`, there is error and `into()` is needed.
    // expected struct `std::boxed::Box`, found struct `std::io::Error`
    // https://users.rust-lang.org/t/boxing-errors-in-result-throws-type-mismatch/36692/2
    //.map_err(|e| e.into())

    println!("Server running at http://{}", endpoint);
    println!("Db test http://{}/api/dataSets", endpoint);
    println!("Db test http://{}/api/dataSets/name-of-data-set", endpoint);
    println!(
        "Db test http://{}/api/dataSetsCategories/dataSets",
        endpoint
    );
    let _ = sys.run();

    Ok(())
}
