// ROCKET_ENV=dev|prod|stage ./wobsite
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::{NamedFile, Redirect};


// BASIC ROUTES

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/files/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("public/").join(file)).ok()
}

#[get("/.well-known/<file..>")]
fn well_known(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new(".well-known/").join(file)).ok()
}

#[get("/gpg")] fn gpg() -> io::Result<NamedFile> {
    NamedFile::open("static/oms.gpg")
}

#[get("/resume")]
fn resume() -> io::Result<NamedFile> {
    NamedFile::open("static/resume.pdf")
}

#[get("/github")]
fn github() -> Redirect {
    Redirect::to("https://github.com/stensonowen")
}



// ERROR HANDLING

#[error(404)]
fn not_found() -> Option<NamedFile> {
	NamedFile::open("static/404.png").ok()
}


fn main() {
    //verify we're in the right directory
    assert!(Path::new("static").is_dir(), "No `static` folder; change directories");
    assert!(Path::new("public").is_dir(), "No `public` folder; change directories");
    
    //run
    rocket::ignite()
        .mount("/", routes![index, files, gpg, resume, github, well_known])
        .catch(errors![not_found])
        .launch();
}


