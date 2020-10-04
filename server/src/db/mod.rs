pub mod models;
pub mod schema;
pub use models::*;
use mysql::{prelude::*, *};
pub use schema::*;

const DB_URI: &str = "mysql://manager:SuperSecureManagerPassword1@localhost:3306/offsite";

pub fn get_connection() -> Result<PooledConn> {
    Pool::new(DB_URI)?.get_conn()
}

pub trait Options {
    fn into_delimited(&self) -> String;
    fn into_filter(&self) -> String;
    fn into_update(&self) -> String;
}

pub trait Update<Changes: Options + Clone> {
    fn update(&mut self, changes: Changes) -> mysql::Result<()> {
        let mut conn = crate::db::get_connection()?;
        let query = changes.into_update();
        conn.query::<Vec<_>, String>(query)?;
        Ok(())
    }
}

pub trait Insert {
    fn insert(&self) -> mysql::Result<Option<i64>>;
}
