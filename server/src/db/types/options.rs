use crate::db::Options;
use schema_proc_macros::*;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct WorkorderOptions {
    pub id: Option<i64>,
    pub active: Option<bool>,
    pub origin: Option<i64>,
    pub travel_status: Option<i64>,
    pub created: Option<i64>,
    #[db_name("quoted")]
    pub quoted_time: Option<i64>,
    #[db_name("workorder_status")]
    pub status: Option<Vec<i64>>,
    pub customer: Option<i64>,
    pub device: Option<i64>,
    pub brief: Option<String>,
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct DeviceOptions {
    pub id: Option<i64>,
    #[db_name("serial_no")]
    pub serial: Option<String>,
    #[db_name("device_name")]
    pub name: Option<String>,
    pub password: Option<String>,
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct StoreOptions {
    pub id: Option<i64>,
    #[db_name("contact_name")]
    pub name: Option<String>,
    pub phone_number: Option<String>,
    #[db_name("email_address")]
    pub email: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct CustomerOptions {
    pub id: Option<i64>,
    #[db_name("customer_name")]
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub store_id: Option<i64>,
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct UserOptions {
    pub id: Option<i64>,
    pub google_id: Option<i128>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct NotesOptions {
    #[db_name("note_id")]
    pub id: Option<i64>,
    #[db_name("wo_key")]
    pub workorder_id: Option<i64>,
}
