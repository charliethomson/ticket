// TODO: Input validation for important fields

use chrono::{prelude::*, serde::ts_seconds};
use mysql::{prelude::*, *};
use serde::{Deserialize, Serialize};
pub type WorkorderTuple = (
    i64,
    i64,
    String,
    DateTime<Utc>,
    Option<DateTime<Utc>>,
    String,
    i64,
    i64,
    String,
);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Workorder {
    pub workorder_id: Option<i64>,
    pub origin: i64,
    pub travel_status: String,
    pub created: DateTime<Utc>,
    pub quoted_time: Option<DateTime<Utc>>,
    pub status: String,
    pub customer: i64,
    pub device: i64,
    pub brief: String,
}
impl From<WorkorderTuple> for Workorder {
    fn from(tuple: WorkorderTuple) -> Self {
        let (
            workorder_id,
            origin,
            travel_status,
            created,
            quoted_time,
            status,
            customer,
            device,
            brief,
        ) = tuple;

        Self {
            workorder_id: Some(workorder_id),
            origin,
            travel_status,
            created,
            quoted_time,
            status,
            customer,
            device,
            brief,
        }
    }
}

pub type DeviceTuple = (i64, String, String, i64, String);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Device {
    pub id: i64,
    pub serial: String,
    pub name: String,
    pub customer_id: i64, // Customer ID
    pub password: String,
}
impl From<DeviceTuple> for Device {
    fn from(tuple: DeviceTuple) -> Self {
        let (id, serial, name, customer_id, password) = tuple;
        Self {
            id,
            serial,
            name,
            customer_id,
            password,
        }
    }
}

pub type StoreTuple = (
    i64,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Store {
    pub id: i64,
    pub name: String,
    pub contact_name: String,
    pub phone_number: String,
    pub email: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}
impl From<StoreTuple> for Store {
    fn from(tuple: StoreTuple) -> Self {
        let (id, name, contact_name, phone_number, email, address, city, state, zip) = tuple;
        Self {
            id,
            name,
            contact_name,
            phone_number,
            email,
            address,
            city,
            state,
            zip,
        }
    }
}

pub type NoteTuple = (
    i64,
    DateTime<Utc>,
    Option<DateTime<Utc>>,
    String,
    Option<String>,
);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Note {
    pub user: i64,
    pub created: DateTime<Utc>,
    pub next_update: Option<DateTime<Utc>>, // TODO ?
    pub contents: String,
    pub location: Option<String>,
}
impl From<NoteTuple> for Note {
    fn from(tuple: NoteTuple) -> Self {
        let (user, created, next_update, contents, location) = tuple;
        Self {
            user,
            created,
            next_update,
            contents,
            location,
        }
    }
}

pub type CustomerTuple = (i64, String, String, String, i64);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub phone_number: String,
    pub email: String,
    pub store_id: i64,
}
impl From<CustomerTuple> for Customer {
    fn from(tuple: CustomerTuple) -> Self {
        let (id, name, phone_number, email, store_id) = tuple;
        Self {
            id,
            name,
            phone_number,
            email,
            store_id,
        }
    }
}

pub type UserTuple = (i64, String, String);

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub phone_number: String,
    // TODO:
}
impl From<UserTuple> for User {
    fn from(tuple: UserTuple) -> Self {
        let (id, name, phone_number) = tuple;
        Self {
            id,
            name,
            phone_number,
        }
    }
}
