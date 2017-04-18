use rocket::response::NamedFile;

use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("../client/dist/index.html")
}

#[allow(unused_variables)]
#[get("/<file..>", rank = 100)]
fn redirect_to_index(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open("../client/dist/index.html")
}

#[get("/js/<file..>")]
fn js(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("../client/dist/js").join(file))
}

#[get("/styles/<file..>")]
fn styles(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("../client/dist/styles").join(file))
}

// TODO: files like `fontawesome-webfont.eot?#iefix&v=4.7.0` are supported on master
// branch of Rocket.
// Remove this in the future.
#[allow(dead_code)]
#[derive(FromForm)]
struct Task<'r> {
    v: &'r str,
}

#[allow(unused_variables)]
#[get("/styles/<file..>?<task>")]
fn styles_with_query(file: PathBuf, task: Task) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("../client/dist/styles").join(file))
}

#[get("/images/<file..>")]
fn images(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("../client/dist/images").join(file))
}
