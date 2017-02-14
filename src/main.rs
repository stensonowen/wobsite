#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
//use rocket::config::{Config, Environment};


#[get("/file/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("public/").join(file)).ok()
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("index.html")
}

fn main() {
    //let config = Config::build(Environment::Production)
    //    .address("0.0.0.0")
    //    .port(8070)
    //    .finalize()
    //    .unwrap()
    //    ;
    //let app = rocket::custom(config, false);
    //app.mount("/", routes![hello, hi]).launch();
    rocket::ignite().mount("/", routes![index, files]).launch();
}
