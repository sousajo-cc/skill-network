pub mod models;
pub mod sanitize_search_string;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::LogsDbConn;
use dotenv::dotenv;
use std::env;

//TODO: no need to connect every time
pub fn establish_connection() -> LogsDbConn {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    LogsDbConn(SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)))
}
