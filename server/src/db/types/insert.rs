use crate::db::schema::{customers, devices, notes, stores, users, workorders};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use webforms::validate::{ValidateError, ValidateForm};

/*
    diesel::insert_into(<table_name>)
        .values(&<tableNew>)
        .execute(&<connection>)
        .unwrap();
*/

#[derive(Serialize, Deserialize, ValidateForm, Insertable)]
#[table_name = "customers"]
pub struct CustomerNew {
    first_name: String,
    last_name: String,
    #[validate(regex = r"^(\+\d{1,2}\s)?(\d{3})?[\s.-]\d{3}[\s.-]\d{4}$")]
    phone_number: String,
    #[validate(email)]
    email_address: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "devices"]
pub struct DeviceNew {
    pub serial_no: String,
    pub device_name: String,
    pub customer: i64, // Customer ID
    pub password: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "notes"]
pub struct NotesNew {
    workorder_id: i64,
    contents: String,
}

#[derive(Serialize, Deserialize, ValidateForm, Insertable)]
#[table_name = "stores"]
pub struct StoreNew {
    contact_name: String,
    #[validate(regex = r"^(\+\d{1,2}\s)?(\d{3})?[\s.-]\d{3}[\s.-]\d{4}$")]
    phone_number: String,
    #[validate(email)]
    email_address: String,
    address: String,
    city: String,
    #[validate(max_length = 2)]
    #[validate(min_length = 2)]
    state: String,
    zip: i32,
}

#[derive(Serialize, Deserialize, ValidateForm, Clone, Insertable)]
#[table_name = "users"]
pub struct UserNew {
    pub google_id: Vec<u8>,
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email_address: String,
}

#[derive(Serialize, Deserialize, ValidateForm, Insertable)]
#[table_name = "workorders"]
pub struct WorkorderNew {
    pub origin: i64,
    pub workorder_status: i32,
    pub travel_status: i32,
    pub quoted: Option<i32>,
    pub location: Option<String>,
    pub customer: i64, // Customer ID
    pub device: i64,   // Device ID
    #[validate(max_length = 144)]
    pub brief: String,
}
