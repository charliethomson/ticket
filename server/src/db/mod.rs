use diesel::mysql::MysqlConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

pub mod schema;
mod types;

pub use types::*;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
