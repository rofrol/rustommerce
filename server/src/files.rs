use actix_web::{fs, FromRequest, HttpRequest};

// use std::io;
use std::path::{Path, PathBuf};

pub fn index(_req: &HttpRequest) -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("../client/dist/index.html")?)
}

// http://stackoverflow.com/questions/2208933/how-do-i-force-a-favicon-refresh
pub fn favicon(_req: &HttpRequest) -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("../client/dist/favicon.ico")?)
}

/*
#[get("/<file..>", rank = 100)]
fn redirect_to_index(file: PathBuf) -> CORS<io::Result<NamedFile>> {
    CORS::any(NamedFile::open("../client/dist/index.html"))
}
*/

pub fn js(req: &HttpRequest) -> actix_web::Result<fs::NamedFile> {
    let file = actix_web::Path::<PathBuf>::extract(req)?.into_inner();
    Ok(fs::NamedFile::open(
        Path::new("../client/dist/js").join(file),
    )?)
}

pub fn styles(req: &HttpRequest) -> actix_web::Result<fs::NamedFile> {
    let file = actix_web::Path::<PathBuf>::extract(req)?.into_inner();
    Ok(fs::NamedFile::open(
        Path::new("../client/dist/styles").join(file),
    )?)
}

/*
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
