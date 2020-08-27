use actix_files as fs;
use actix_web::{web, HttpRequest};

// use std::io;
use std::path::{Path, PathBuf};

pub async fn index(_req: HttpRequest) -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("../client/dist/index.html")?)
}

// http://stackoverflow.com/questions/2208933/how-do-i-force-a-favicon-refresh
pub async fn favicon() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("../client/dist/favicon.ico")?)
}

pub async fn js(file: web::Path<PathBuf>) -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(
        Path::new("../client/dist/js").join(file.into_inner()),
    )?)
}

pub async fn styles(file: web::Path<PathBuf>) -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(
        Path::new("../client/dist/styles").join(file.into_inner()),
    )?)
}
