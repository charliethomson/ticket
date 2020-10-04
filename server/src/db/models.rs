// TODO: Input validation for important fields

use schema_proc_macros::*;
use serde::{Deserialize, Serialize};
pub type WorkorderTuple = (i64, i64, String, i64, Option<i64>, String, i64, i64, String);

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct WorkorderResponse {
    pub workorder_id: i64,
    pub origin: Store,
    pub travel_status: String,
    pub created: i64,
    pub quoted_time: Option<i64>,
    pub status: String,
    pub customer: Customer,
    pub device: Device,
    pub brief: String,
    pub notes: Vec<Note>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Insert)]
pub struct Workorder {
    #[db_name("id")]
    pub workorder_id: i64,
    pub origin: i64,
    pub travel_status: String,
    pub created: i64,
    #[db_name("quoted")]
    pub quoted_time: Option<i64>,
    #[db_name("workorder_status")]
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
            workorder_id,
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

pub type NoteTuple = (String, i64, i64, Option<i64>);

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Note {
    pub user: i64,
    pub created: i64,
    pub next_update: Option<i64>,
    pub contents: String,
}
impl From<NoteTuple> for Note {
    fn from(tuple: NoteTuple) -> Self {
        let (contents, user, created, next_update) = tuple;
        Self {
            user,
            created,
            next_update,
            contents,
        }
    }
}

pub type CustomerTuple = (i64, String, String, String, i64);

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

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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
