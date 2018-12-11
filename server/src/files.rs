use actix_web::{fs, HttpRequest};

/*
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> CORS<io::Result<NamedFile>> {
    CORS::any(NamedFile::open("../client/dist/index.html"))
}
*/

// http://stackoverflow.com/questions/2208933/how-do-i-force-a-favicon-refresh
pub fn favicon(_req: &HttpRequest) -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("../client/dist/favicon.ico")?)
}

/*
#[allow(unused_variables)]
#[get("/<file..>", rank = 100)]
fn redirect_to_index(file: PathBuf) -> CORS<io::Result<NamedFile>> {
    CORS::any(NamedFile::open("../client/dist/index.html"))
}

#[get("/js/<file..>")]
fn js(file: PathBuf) -> CORS<io::Result<NamedFile>> {
    CORS::any(NamedFile::open(Path::new("../client/dist/js").join(file)))
}

#[get("/styles/<file..>")]
fn styles(file: PathBuf) -> CORS<io::Result<NamedFile>> {
    CORS::any(NamedFile::open(Path::new("../client/dist/styles").join(file)))
}

// TODO: files like `fontawesome-webfont.eot?#iefix&v=4.7.0` are supported on master
// branch of Rocket.
// Remove this in the future.
#[allow(dead_code)]
#[derive(FromForm)]
struct V<'r> {
    v: &'r str,
}

#[allow(unused_variables)]
#[get("/styles/<file..>?<v>")]
fn styles_with_query(file: PathBuf, v: V) -> CORS<io::Result<NamedFile>> {
    CORS::any(NamedFile::open(Path::new("../client/dist/styles").join(file)))
}

#[get("/images/<file..>")]
fn images(file: PathBuf) -> CORS<io::Result<NamedFile>> {
    CORS::any(NamedFile::open(Path::new("../client/dist/images").join(file)))
}
*/
