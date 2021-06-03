pub mod models;
pub mod schema;
pub mod sanitize_search_string;

use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

//TODO: no need to connect every time
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}