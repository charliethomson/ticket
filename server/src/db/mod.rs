#![allow(clippy::expect_fun_call)]
use diesel::{mysql::MysqlConnection, prelude::*};
use dotenv::dotenv;
use std::env;

pub mod schema;
mod types;

pub use types::*;

no_arg_sql_function!(last_insert_id, diesel::types::Bigint);

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
pub fn last_inserted(conn: &MysqlConnection) -> i64 {
    let a = diesel::select(last_insert_id).first(conn).unwrap();
    println!("{}", a);
    a
}

pub trait IntoQuery<T>
where
    T: diesel::Table + diesel::query_builder::AsQuery,
    T::Query: diesel::query_dsl::methods::BoxedDsl<'static, diesel::mysql::Mysql>,
{
    fn into_query(self) -> diesel::helper_types::IntoBoxed<'static, T, diesel::mysql::Mysql>;
}
