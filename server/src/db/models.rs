// TODO: Input validation for important fields

use schema_proc_macros::*;
use serde::{Deserialize, Serialize};

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct WorkorderResponse {
    pub workorder_id: i64,
    pub active: bool,
    pub origin: Store,
    pub created: i64,
    pub quoted_time: Option<i64>,
    pub status: i64,
    pub travel_status: i64,
    pub location: Option<String>,
    pub customer: Customer,
    pub device: Device,
    pub brief: String,
    pub notes: Vec<NoteResponse>,
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Insert)]
pub struct Workorder {
    #[db_name("id")]
    pub workorder_id: i64,
    pub active: bool,
    pub origin: i64,
    pub created: i64,
    #[db_name("quoted")]
    pub quoted_time: Option<i64>,
    #[db_name("workorder_status")]
    pub status: i64,
    pub travel_status: i64,
    pub location: Option<String>,
    pub customer: i64,
    pub device: i64,
    pub brief: String,
}

#[build_tuple]
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

#[build_tuple]
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

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Note {
    pub contents: String,
    pub user: i64,
    pub created: i64,
    pub next_update: Option<i64>,
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct NoteResponse {
    pub user: UserResponse,
    pub contents: String,
    pub created: i64,
    pub next_update: Option<i64>,
}
impl std::convert::TryFrom<Note> for NoteResponse {
    type Error = mysql::Error;
    fn try_from(note: Note) -> Result<Self, Self::Error> {
        Ok(NoteResponse {
            // TODO
            user: match User::find(crate::db::schema::UserOptions {
                id: Some(note.user),
                ..crate::db::schema::UserOptions::default()
            })? {
                Some(users) => users.get(0).expect("see below smile").clone().into(),
                None => panic!("This should never happen, my db should work :)"),
            },
            contents: note.contents,
            created: note.created,
            next_update: note.next_update,
        })
    }
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Insert)]
pub struct Customer {
    pub id: i64,
    #[db_name("customer_name")]
    pub name: String,
    pub phone_number: String,
    #[db_name("email_address")]
    pub email: String,
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub google_id: String,
    pub name: String,
    pub email: String,
}

#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
}
impl From<User> for UserResponse {
    fn from(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
