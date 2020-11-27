use super::models::{Customer, Device, Note, Store, User, Workorder};
use crate::db::{
    establish_connection,
    schema::{notes::dsl::notes, users},
};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WorkorderResponse {
    pub workorder_id: i64,
    pub active: bool,
    pub origin: Store,
    pub created: i32,
    pub quoted_time: Option<i32>,
    pub status: i32,
    pub travel_status: i32,
    pub location: Option<String>,
    pub customer: Customer,
    pub device: Device,
    pub brief: String,
    pub notes: Vec<NoteResponse>,
}
impl From<Workorder> for WorkorderResponse {
    fn from(wo: Workorder) -> WorkorderResponse {
        use crate::db::schema::{customers, devices, stores, workorders};

        let connection = crate::db::establish_connection();

        // Get the store, customer and device
        let (_, origin, customer, device) = workorders::dsl::workorders
            .inner_join(stores::table)
            .inner_join(customers::table)
            .inner_join(devices::table)
            .filter(workorders::id.eq(wo.id))
            .first::<(Workorder, Store, Customer, Device)>(&connection)
            .expect("Failed");

        // Get notes
        let workorder_notes: Vec<NoteResponse> = Note::belonging_to(&wo)
            .load::<Note>(&connection)
            .expect("Failed to get notes")
            .into_iter()
            .map(NoteResponse::from)
            .collect();

        WorkorderResponse {
            workorder_id: wo.id,
            active: wo.active,
            origin,
            created: wo.created,
            quoted_time: wo.quoted,
            status: wo.workorder_status,
            travel_status: wo.travel_status,
            location: wo.location,
            customer,
            device,
            brief: wo.brief,
            notes: workorder_notes,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub user: UserResponse,
    pub contents: String,
    pub posted: i32,
    pub next_update: Option<i32>,
}
impl From<Note> for NoteResponse {
    fn from(note: Note) -> Self {
        let (_, user): (_, User) = notes
            .inner_join(users::table)
            .first::<(Note, User)>(&establish_connection())
            .expect("Failed to get user");
        NoteResponse {
            user: user.into(),
            contents: note.contents,
            posted: note.posted,
            next_update: note.next_update,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
}
impl From<User> for UserResponse {
    fn from(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email_address: user.email_address,
        }
    }
}
