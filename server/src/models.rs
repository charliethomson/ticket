// TODO: Input validation for important fields

use chrono::{prelude::*, serde::ts_seconds};
use mongodb::bson::document::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Workorder {
    id: u64,
    origin: Store,
    travel_status: String,
    delivered_by: String,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    last_update: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    quoted_time: DateTime<Utc>,
    status: i32,
    customer: Customer,
    device: Device,
    notes: Vec<Note>,
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    id: u64,
    serial: String,
    customer: Customer,
    workorders: Vec<Workorder>,
}

#[derive(Serialize, Deserialize)]
pub struct Store {
    id: u64,
    name: String,
    contact_name: String,
    phone_number: String,
    workorders: Vec<Workorder>,
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: u64,
    user: User,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,

    #[serde(with = "ts_seconds")]
    next_update: DateTime<Utc>, // TODO ?

    contents: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
pub struct Customer {
    id: u64,
    name: String,
    phone_number: String,
    email: String,
    devices: Vec<Device>,
    workorders: Vec<Workorder>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
    phone_number: String,
    queue: Vec<Workorder>,
    // TODO:
}
