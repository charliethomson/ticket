// TODO: Input validation for important fields

use schema_proc_macros::*;
use serde::{Deserialize, Serialize};

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct WorkorderResponse {
    pub workorder_id: i64,
    pub active: bool,
    pub origin: Store,
    pub created: i64,
    pub quoted_time: Option<i64>,
    pub status: i64,
    pub travel_status: i64,
    pub location: Option<String>,
    pub customer: Customer,
    pub device: Device,
    pub brief: String,
    pub notes: Vec<Note>,
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Insert)]
pub struct Workorder {
    #[db_name("id")]
    pub workorder_id: i64,
    pub active: bool,
    pub origin: i64,
    pub created: i64,
    #[db_name("quoted")]
    pub quoted_time: Option<i64>,
    #[db_name("workorder_status")]
    pub status: i64,
    pub travel_status: i64,
    pub location: Option<String>,
    pub customer: i64,
    pub device: i64,
    pub brief: String,
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Insert)]
pub struct Device {
    pub id: i64,
    #[db_name("serial_no")]
    pub serial: String,
    #[db_name("device_name")]
    pub name: String,
    #[db_name("customer")]
    pub customer_id: i64, // Customer ID
    pub password: String,
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Insert)]
pub struct Store {
    pub id: i64,
    #[db_name("store_name")]
    pub name: String,
    pub contact_name: String,
    pub phone_number: String,
    #[db_name("email_address")]
    pub email: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Note {
    pub contents: String,
    pub user: i64,
    pub created: i64,
    pub next_update: Option<i64>,
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Insert)]
pub struct Customer {
    pub id: i64,
    #[db_name("customer_name")]
    pub name: String,
    pub phone_number: String,
    #[db_name("email_address")]
    pub email: String,
    pub store_id: i64,
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub phone_number: String,
    // TODO:
}
