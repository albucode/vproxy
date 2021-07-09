use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

pub struct Database;

impl Database {
    pub fn connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        PgConnection::establish(&database_url).expect("Error connecting to the database.")
    }
}
