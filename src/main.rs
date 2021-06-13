mod models;
mod schema;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Connection;
use dotenv::dotenv;
use models::*;
use rocket::fs::NamedFile;
use std::env;
use std::path::Path;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect("Error connecting to the database.")
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    use crate::schema::videos::dsl::*;

    let connection = establish_connection();

    let results = videos.limit(5).load::<Video>(&connection);

    println!("{:?}", results);

    NamedFile::open(Path::new("./storage/test.txt")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
