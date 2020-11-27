use serde::Deserialize;

#[derive(Deserialize, Queryable)]
pub struct WorkorderFilter {
    pub id: Option<i64>,
    pub active: Option<bool>,
    pub origin: Option<i64>,
    pub created: Option<i32>,
    pub quoted: Option<i32>,
    pub workorder_status: Option<i32>,
    pub travel_status: Option<i32>,
    pub location: Option<String>,
    pub customer: Option<i64>,
    pub device: Option<i64>,
    pub brief: Option<String>,
}

#[derive(Deserialize, Queryable)]
pub struct DeviceFilter {
    pub id: Option<i64>,
    pub serial_no: Option<String>,
    pub device_name: Option<String>,
    pub customer: Option<i64>, // Customer ID
    pub password: Option<String>,
}

#[derive(Deserialize, Queryable)]
pub struct StoreFilter {
    pub id: Option<i64>,
    pub contact_name: Option<String>,
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<i32>,
}

#[derive(Deserialize, Queryable)]
pub struct CustomerFilter {
    pub id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub email_address: Option<String>,
}

#[derive(Deserialize, Queryable)]
pub struct UserFilter {
    pub id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_address: Option<String>,
}

#[derive(Deserialize, Queryable)]
pub struct NoteFilter {
    pub id: Option<i64>,
    pub workorder_id: Option<i64>,
}
