pub mod schema;
pub mod types;
use mysql::{prelude::*, *};
pub use schema::*;
pub use types::*;

const DB_URI: &str = "mysql://manager:SuperSecureManagerPassword1@localhost:3306/offsite";

pub fn get_connection() -> Result<PooledConn> {
    Pool::new(DB_URI)?.get_conn()
}

// impl Update<WorkorderOptions> for WorkorderResponse {}
// impl Update<DeviceOptions> for Device {}
// impl Update<StoreOptions> for Store {}
// impl Update<CustomerOptions> for Customer {}
// impl Update<UserOptions> for User {}

pub trait Exists: HasTable + Filter {
    fn exists(&self) -> Option<i64>;
}

pub trait Insert: HasTable {
    fn insert(&self) -> mysql::Result<Option<i64>>;
}

pub trait Filter: HasTable {
    fn into_delimited(&self) -> String;
    fn into_filter(&self) -> String;
    fn into_update(&self) -> String;
}

pub trait Find<Options: Filter>: HasTable + Sized {
    fn find(options: Options) -> mysql::Result<Option<Vec<Self>>>;
}

pub trait Options {}

pub trait Update<Changes: Filter>: HasTable {
    fn update(&mut self, changes: Changes) -> mysql::Result<()> {
        let mut conn = crate::db::get_connection()?;
        let query = changes.into_update();
        conn.query::<Vec<_>, String>(query)?;
        Ok(())
    }
}

pub trait HasTable {
    const TABLE_NAME: &'static str;
}
