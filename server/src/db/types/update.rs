use crate::db::schema::*;
use serde::Deserialize;

#[derive(AsChangeset, Deserialize)]
#[table_name = "workorders"]
pub struct WorkorderUpdate {
    pub id: i64,
    pub active: Option<bool>,
    pub origin: Option<i64>,
    pub created: Option<i32>,
    pub quoted: Option<Option<i32>>,
    pub workorder_status: Option<i32>,
    pub travel_status: Option<i32>,
    pub location: Option<Option<String>>,
    pub customer: Option<i64>,
    pub device: Option<i64>,
    pub brief: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "devices"]
pub struct DeviceUpdate {
    pub id: i64,
    pub serial_no: Option<String>,
    pub device_name: Option<String>,
    pub customer: Option<i64>, // Customer ID
    pub password: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "stores"]
pub struct StoreUpdate {
    pub id: i64,
    pub contact_name: Option<String>,
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<i32>,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "customers"]
pub struct CustomerUpdate {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "users"]
pub struct UserUpdate {
    pub id: i64,
    pub google_id: Option<Vec<u8>>,
    pub portal_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_address: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "notes"]
pub struct NotesUpdate {
    pub id: i64,
    pub workorder_id: Option<i64>,
}
