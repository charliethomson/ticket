// TODO: Input validation for important fields

use chrono::{prelude::*, serde::ts_seconds};
use mongodb::bson::document::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Workorder {
    pub id: i64,
    pub origin: Store,
    pub travel_status: String,
    pub delivered_by: String,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub last_update: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub quoted_time: DateTime<Utc>,
    pub status: i32,
    pub customer: Customer,
    pub device: Device,
    pub notes: Vec<Note>,
}
impl std::convert::TryFrom<Document> for Workorder {
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Device {
    pub id: i64,
    pub serial: String,
    pub customer: Customer,
    pub workorders: Vec<Workorder>,
}
impl std::convert::TryFrom<Document> for Device {
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Store {
    pub id: i64,
    pub name: String,
    pub contact_name: String,
    pub phone_number: u32,
    pub workorders: Vec<Workorder>,
}
impl std::convert::TryFrom<Document> for Store {
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
impl std::convert::TryFrom<Document> for Note {
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub phone_number: u32,
    pub email: String,
    pub devices: Vec<Device>,
    pub workorders: Vec<Workorder>,
}
impl std::convert::TryFrom<Document> for Customer {
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub phone_number: String,
    pub queue: Vec<Workorder>,
    // TODO:
}
impl std::convert::TryFrom<Document> for User {
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
}
