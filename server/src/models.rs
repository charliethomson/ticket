// TODO: Input validation for important fields

use chrono::{prelude::*, serde::ts_seconds};
use mongodb::bson::document::Document;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Workorder {
    pub id: i64,
    pub origin: Store,
    pub travel_status: String,
    // TODO: pub delivered_by: String,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub quoted_time: DateTime<Utc>,
    pub status: String,
    pub customer: Customer,
    pub device: Device,
    pub brief: String,
    pub notes: Vec<Note>,
}
impl TryFrom<Document> for Workorder {
    type Error = ();

    fn try_from(doc: Document) -> Result<Workorder, Self::Error> {
        match Self::deserialize(mongodb::bson::Deserializer::new(doc.into())).ok() {
            Some(item) => Ok(item),
            None => Err(()),
        }
    }
}
impl Workorder {
    pub fn into_document(self) -> Document {
        match self
            .serialize(mongodb::bson::Serializer::new())
            .expect("\n\n\n\n\n\nSER ERROR!")
        {
            mongodb::bson::Bson::Document(doc) => doc,
            _ => unreachable!(),
        }
    }
    pub async fn try_from_id(id: i64) -> Option<Self> {
        let mut filter = Document::new();
        filter.insert("id", id);
        Workorder::try_from(
            crate::db::get_collection("workorders")
                .await
                .ok()?
                .find_one(Some(filter), None)
                .await
                .ok()??,
        )
        .ok()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Device {
    pub id: i64,
    pub serial: String,
    pub name: String,
    pub customer: Customer,
    pub workorders: Vec<Workorder>,
    pub password: String,
}
impl TryFrom<Document> for Device {
    type Error = ();

    fn try_from(doc: Document) -> Result<Device, Self::Error> {
        match Self::deserialize(mongodb::bson::Deserializer::new(doc.into())).ok() {
            Some(item) => Ok(item),
            None => Err(()),
        }
    }
}
impl Device {
    pub fn into_document(self) -> Document {
        match self
            .serialize(mongodb::bson::Serializer::new())
            .expect("\n\n\n\n\n\nSER ERROR!")
        {
            mongodb::bson::Bson::Document(doc) => doc,
            _ => unreachable!(),
        }
    }
    pub async fn try_from_id(id: i64) -> Option<Self> {
        let mut filter = Document::new();
        filter.insert("id", id);
        Device::try_from(
            crate::db::get_collection("devices")
                .await
                .ok()?
                .find_one(Some(filter), None)
                .await
                .ok()??,
        )
        .ok()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Store {
    pub id: i64,
    pub name: String,
    pub contact_name: String,
    pub phone_number: i32,
    pub email: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: i32,
    // pub workorders: Vec<Workorder>,
}
impl TryFrom<Document> for Store {
    type Error = ();

    fn try_from(doc: Document) -> Result<Store, Self::Error> {
        match Self::deserialize(mongodb::bson::Deserializer::new(doc.into())).ok() {
            Some(item) => Ok(item),
            None => Err(()),
        }
    }
}
impl Store {
    pub fn into_document(self) -> Document {
        match self
            .serialize(mongodb::bson::Serializer::new())
            .expect("\n\n\n\n\n\nSER ERROR!")
        {
            mongodb::bson::Bson::Document(doc) => doc,
            _ => unreachable!(),
        }
    }
    pub async fn try_from_id(id: i64) -> Option<Self> {
        let mut filter = Document::new();
        filter.insert("id", id);
        Store::try_from(
            crate::db::get_collection("stores")
                .await
                .ok()?
                .find_one(Some(filter), None)
                .await
                .ok()??,
        )
        .ok()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    pub id: i64,
    pub user: User,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,

    #[serde(with = "ts_seconds")]
    pub next_update: DateTime<Utc>, // TODO ?

    pub contents: String,
    pub location: String,
}
impl TryFrom<Document> for Note {
    type Error = ();

    fn try_from(doc: Document) -> Result<Note, Self::Error> {
        match Self::deserialize(mongodb::bson::Deserializer::new(doc.into())).ok() {
            Some(item) => Ok(item),
            None => Err(()),
        }
    }
}
impl Note {
    pub fn into_document(self) -> Document {
        match self
            .serialize(mongodb::bson::Serializer::new())
            .expect("\n\n\n\n\n\nSER ERROR!")
        {
            mongodb::bson::Bson::Document(doc) => doc,
            _ => unreachable!(),
        }
    }
    pub async fn try_from_id(id: i64) -> Option<Self> {
        let mut filter = Document::new();
        filter.insert("id", id);
        Note::try_from(
            crate::db::get_collection("Notes")
                .await
                .ok()?
                .find_one(Some(filter), None)
                .await
                .ok()??,
        )
        .ok()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub phone_number: i32,
    pub email: String,
    pub storeid: i64,
    pub devices: Vec<i64>,    // Device IDs
    pub workorders: Vec<i64>, // Workorder IDs
}
impl TryFrom<Document> for Customer {
    type Error = ();

    fn try_from(doc: Document) -> Result<Customer, Self::Error> {
        match Self::deserialize(mongodb::bson::Deserializer::new(doc.into())).ok() {
            Some(item) => Ok(item),
            None => Err(()),
        }
    }
}
impl Customer {
    pub fn into_document(self) -> Document {
        match self
            .serialize(mongodb::bson::Serializer::new())
            .expect("\n\n\n\n\n\nSER ERROR!")
        {
            mongodb::bson::Bson::Document(doc) => doc,
            _ => unreachable!(),
        }
    }
    pub async fn try_from_id(id: i64) -> Option<Self> {
        let mut filter = Document::new();
        filter.insert("id", id);
        Customer::try_from(
            crate::db::get_collection("Customers")
                .await
                .ok()?
                .find_one(Some(filter), None)
                .await
                .ok()??,
        )
        .ok()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub phone_number: String,
    pub queue: Vec<Workorder>,
    // TODO:
}
impl TryFrom<Document> for User {
    type Error = ();

    fn try_from(doc: Document) -> Result<User, Self::Error> {
        match Self::deserialize(mongodb::bson::Deserializer::new(doc.into())).ok() {
            Some(item) => Ok(item),
            None => Err(()),
        }
    }
}
impl User {
    pub fn into_document(self) -> Document {
        match self
            .serialize(mongodb::bson::Serializer::new())
            .expect("\n\n\n\n\n\nSER ERROR!")
        {
            mongodb::bson::Bson::Document(doc) => doc,
            _ => unreachable!(),
        }
    }
    pub async fn try_from_id(id: i64) -> Option<Self> {
        let mut filter = Document::new();
        filter.insert("id", id);
        User::try_from(
            crate::db::get_collection("users")
                .await
                .ok()?
                .find_one(Some(filter), None)
                .await
                .ok()??,
        )
        .ok()
    }
}
