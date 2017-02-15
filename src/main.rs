// ROCKET_ENV=dev|prod|stage ./wobsite
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;


#[get("/file/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("public/").join(file)).ok()
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[error(404)]
fn not_found() -> Option<NamedFile> {
	NamedFile::open("static/404.png").ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .catch(errors![not_found])
        .launch();
}


