#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use std::path::Path;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./storage/test.txt")).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
