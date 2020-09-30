pub mod models;
pub mod schema;
pub use models::*;
pub use mysql::{prelude::*, *};
pub use schema::*;

const DB_URI: &'static str = "mysql://manager:SuperSecureManagerPassword1@localhost:3306/offsite";

pub fn get_connection() -> Result<PooledConn> {
    Pool::new(DB_URI)?.get_conn()
}
