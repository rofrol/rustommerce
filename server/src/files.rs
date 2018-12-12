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
