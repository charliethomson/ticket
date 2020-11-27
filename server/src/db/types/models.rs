use crate::db::schema::*;
use serde::Serialize;

#[derive(Serialize, Debug, Identifiable, Queryable)]
pub struct Customer {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email_address: String,
}

#[derive(Serialize, Debug, Identifiable, Queryable)]
pub struct Device {
    pub id: i64,
    pub serial_no: String,
    pub device_name: String,
    pub customer: i64,
    pub password: Option<String>,
}

#[derive(Serialize, Debug, Identifiable, Queryable, Associations)]
#[belongs_to(Workorder)]
pub struct Note {
    pub id: i64,
    pub workorder_id: i64,
    pub contents: String,
    pub user: i64,
    pub posted: i32,
    pub next_update: Option<i32>,
}

#[derive(Serialize, Debug, Identifiable, Queryable)]
pub struct Store {
    pub id: i64,
    pub contact_name: String,
    pub phone_number: String,
    pub email_address: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: i32,
}

#[derive(Serialize, Debug, Identifiable, Queryable)]
pub struct User {
    pub id: i64,
    pub google_id: Option<Vec<u8>>,
    pub portal_id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
}

#[derive(Serialize, Debug, Identifiable, Queryable)]
pub struct Workorder {
    pub id: i64,
    pub active: bool,
    pub origin: i64,
    pub created: i32,
    pub quoted: Option<i32>,
    pub workorder_status: i32,
    pub travel_status: i32,
    pub location: Option<String>,
    pub customer: i64,
    pub device: i64,
    pub brief: String,
}
