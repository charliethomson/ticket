use crate::db::types::{Customer, Device, Note, Store, User, UserOptions};
use schema_proc_macros::{build_tuple, table_name};
use serde::{Deserialize, Serialize};

#[table_name(workorders)]
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

#[table_name(notes)]
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
            user: match User::find(UserOptions {
                id: Some(note.user),
                ..UserOptions::default()
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
#[table_name(users)]
#[build_tuple]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
impl From<User> for UserResponse {
    fn from(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        }
    }
}
