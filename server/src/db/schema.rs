// TODO: Convert `update` to a proc macro

use crate::db::{models::*, Options, Update};
use crate::routes::users::UserNew;
use mysql::{prelude::*, *};
use schema_proc_macros::*;
use serde::Deserialize;
use std::convert::TryFrom;

// TODO: Validation

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct WorkorderOptions {
    pub id: Option<i64>,
    pub active: Option<bool>,
    pub origin: Option<i64>,
    pub travel_status: Option<i64>,
    pub created: Option<i64>,
    #[db_name("quoted")]
    pub quoted_time: Option<i64>,
    #[db_name("workorder_status")]
    pub status: Option<Vec<i64>>,
    pub customer: Option<i64>,
    pub device: Option<i64>,
    pub brief: Option<String>,
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct DeviceOptions {
    pub id: Option<i64>,
    #[db_name("serial_no")]
    pub serial: Option<String>,
    #[db_name("device_name")]
    pub name: Option<String>,
    pub password: Option<String>,
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct StoreOptions {
    pub id: Option<i64>,
    #[db_name("contact_name")]
    pub name: Option<String>,
    pub phone_number: Option<String>,
    #[db_name("email_address")]
    pub email: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct CustomerOptions {
    pub id: Option<i64>,
    #[db_name("customer_name")]
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub store_id: Option<i64>,
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct UserOptions {
    pub id: Option<i64>,
    pub google_id: Option<i128>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct NotesOptions {
    #[db_name("note_id")]
    pub id: Option<i64>,
    #[db_name("wo_key")]
    pub workorder_id: Option<i64>,
}

impl Workorder {
    pub fn find(filter: WorkorderOptions) -> mysql::Result<Vec<WorkorderResponse>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        dbg!(filter.clone());
        let query = format!(
            "select id from workorders{};",
            if !filter.is_empty() {
                format!(" where {}", filter)
            } else {
                "".to_string()
            }
        );
        let ids: Vec<i64> = conn.query(query)?;

        let wos = ids
            .iter()
            .map(|id| Workorder::by_id(*id))
            .filter(|wo| {
                wo.is_ok()
                    && wo.as_ref().unwrap().is_some()
                    && wo.as_ref().unwrap().as_ref().unwrap().is_ok()
            })
            .map(|wo| wo.unwrap().unwrap().unwrap())
            .collect::<Vec<WorkorderResponse>>();
        if !wos.is_empty() {
            Ok(wos)
        } else {
            Ok(vec![])
        }
    }

    pub fn by_id(id: i64) -> mysql::Result<Option<std::result::Result<WorkorderResponse, String>>> {
        let mut conn = crate::db::get_connection()?;
        if let Some((
            id,
            active,
            store_id,
            created,
            quoted,
            status,
            travel_status,
            location,
            customer_id,
            device_id,
            brief,
        )) = conn.query_first::<WorkorderTuple, String>(format!(
            "select * from workorders where workorders.id={}",
            id
        ))? {
            let origin = match Store::by_id(store_id)? {
                Some(store) => store,
                _ => {
                    return Ok(Some(Err(format!(
                        "Failed to get Store from id {}",
                        store_id
                    ))))
                }
            };
            let device = match Device::by_id(device_id)? {
                Some(device) => device,
                _ => {
                    return Ok(Some(Err(format!(
                        "Failed to get Device from id {}",
                        device_id
                    ))))
                }
            };
            let customer = match Customer::by_id(customer_id)? {
                Some(customer) => customer,
                _ => {
                    return Ok(Some(Err(format!(
                        "Failed to get Customer from id {}",
                        customer_id
                    ))))
                }
            };
            let notes = match Note::all_for_wo(id)? {
                Some(notes) => notes
                    .iter()
                    .cloned()
                    .map(NoteResponse::try_from)
                    .filter(|res| res.is_ok())
                    .map(|res| res.unwrap().clone())
                    .collect(),
                _ => return Ok(Some(Err(format!("Failed to get Notes from id {}", id)))),
            };

            Ok(Some(Ok(WorkorderResponse {
                workorder_id: id,
                active,
                origin,
                travel_status,
                created,
                quoted_time: quoted,
                status,
                location,
                customer,
                device,
                brief,
                notes,
            })))
        } else {
            Ok(None)
        }
    }
}

// TODO
impl Update<WorkorderOptions> for WorkorderResponse {}
impl Update<DeviceOptions> for Device {}
impl Update<StoreOptions> for Store {}
impl Update<CustomerOptions> for Customer {}
impl Update<UserOptions> for User {}

impl Device {
    pub fn find(filter: DeviceOptions) -> mysql::Result<Option<Vec<Self>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        let devices: Vec<Device> = conn
            .query::<i64, String>(format!(
                "select id from devices{}",
                if !filter.is_empty() {
                    format!(" where {}", filter)
                } else {
                    "".to_string()
                }
            ))?
            .iter()
            // TODO: BIG BOI
            .map(|id| Device::by_id(*id))
            .filter(|res| res.is_ok() && res.as_ref().unwrap().is_some())
            .map(|resop| resop.unwrap().unwrap())
            .collect();

        Ok(if devices.is_empty() {
            None
        } else {
            Some(devices)
        })
    }

    pub fn by_id(id: i64) -> mysql::Result<Option<Self>> {
        let mut conn = crate::db::get_connection()?;
        if let Some(device_tuple) = conn.query_first::<DeviceTuple, String>(format!(
            "select * from devices where devices.id={}",
            id
        ))? {
            Ok(Some(Device::from(device_tuple)))
        } else {
            Ok(None)
        }
    }
}

impl Store {
    // TODO: Make this proc macro
    pub fn find(filter: StoreOptions) -> mysql::Result<Option<Vec<Self>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        let stores: Vec<Store> = conn
            .query::<i64, String>(format!(
                "select id from stores{}",
                if !filter.is_empty() {
                    format!(" where {}", filter)
                } else {
                    "".to_string()
                }
            ))?
            .iter()
            // TODO: BIG BOI
            .map(|id| Store::by_id(*id))
            .filter(|res| res.is_ok() && res.as_ref().unwrap().is_some())
            .map(|resop| resop.unwrap().unwrap())
            .collect();

        Ok(if stores.is_empty() {
            None
        } else {
            Some(stores)
        })
    }

    pub fn by_id(id: i64) -> mysql::Result<Option<Self>> {
        let mut conn = crate::db::get_connection()?;
        if let Some(store_tuple) = conn.query_first::<StoreTuple, String>(format!(
            "select * from stores where stores.id={}",
            id
        ))? {
            Ok(Some(Store::from(store_tuple)))
        } else {
            Ok(None)
        }
    }
}

impl Customer {
    pub fn find(filter: CustomerOptions) -> mysql::Result<Option<Vec<Self>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        let customers: Vec<Customer> = conn
            .query::<i64, String>(format!(
                "select id from customers{}",
                if !filter.is_empty() {
                    format!(" where {}", filter)
                } else {
                    "".to_string()
                }
            ))?
            .iter()
            // TODO: BIG BOI
            .map(|id| Customer::by_id(*id))
            .filter(|res| res.is_ok() && res.as_ref().unwrap().is_some())
            .map(|resop| resop.unwrap().unwrap())
            .collect();

        Ok(if customers.is_empty() {
            None
        } else {
            Some(customers)
        })
    }

    pub fn by_id(id: i64) -> mysql::Result<Option<Self>> {
        let mut conn = crate::db::get_connection()?;
        if let Some(customer_tuple) = conn.query_first::<CustomerTuple, String>(format!(
            "select * from customers where customers.id={}",
            id
        ))? {
            Ok(Some(Customer::from(customer_tuple)))
        } else {
            Ok(None)
        }
    }
}

impl User {
    pub fn insert(user: UserNew) -> mysql::Result<i64> {
        let mut conn = crate::db::get_connection()?;
        match conn.exec_drop(
            "insert into users (
                google_id,
                first_name,
                last_name,
                email
            ) values (
                :google_id,
                :first_name,
                :last_name,
                :email
            );",
            params! {
                "google_id" => user.google_id,
                "first_name" => user.first_name.clone(),
                "last_name" => user.last_name.clone(),
                "email" => user.email.clone(),
            },
        ) {
            Ok(_) => Ok(conn
                .query_first::<i64, String>("SELECT max(LAST_INSERT_ID(id)) FROM users".to_owned())?
                .unwrap()),
            // Duplicate user
            Err(mysql::Error::MySqlError(mysql::error::MySqlError { code: 1062, .. })) => {
                Ok(User::find(UserOptions {
                    google_id: Some(user.google_id),
                    ..UserOptions::default()
                })?
                // Unwraps are safe because of the error type, we _will_ find something
                // filtering by the (unique) google_id.
                .unwrap()
                .get(0)
                .unwrap()
                .id)
            }
            Err(e) => Err(e),
        }
    }

    pub fn find(filter: UserOptions) -> mysql::Result<Option<Vec<UserResponse>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        let query = format!(
            "select id from users{};",
            if !filter.is_empty() {
                format!(" where {}", filter)
            } else {
                "".to_string()
            }
        );
        let ids: Vec<i64> = conn.query(query)?;

        // TODO (this and also in Workorders)
        let users = ids
            .iter()
            .map(|id| User::by_id(*id))
            .filter(|user| user.is_ok() && user.as_ref().unwrap().is_some())
            .map(|user| user.unwrap().unwrap().into())
            .collect::<Vec<UserResponse>>();
        if !users.is_empty() {
            Ok(Some(users))
        } else {
            Ok(None)
        }
    }

    pub fn by_id(id: i64) -> mysql::Result<Option<Self>> {
        let mut conn = crate::db::get_connection()?;
        if let Some(user_tuple) = conn.query_first::<UserTuple, String>(format!(
            "select * from users where users.id={}",
            id
        ))? {
            Ok(Some(User::from(user_tuple)))
        } else {
            Ok(None)
        }
    }
}

impl Note {
    pub fn insert(&self, workorder_id: i64) -> mysql::Result<i64> {
        let mut conn = crate::db::get_connection()?;
        conn.exec_drop(
            "insert into notes
        (   
            wo_key,
            contents,
            user,
            posted,
            next_update
        ) values (
            :wo_key,
            :contents,
            :user,
            :posted,
            :next_update
        );",
            params! {
                "wo_key" => workorder_id,
                "contents" => self.contents.clone(),
                "user" => self.user,
                "posted" => self.created,
                "next_update" => self.next_update
            },
        )?;

        Ok(conn
            .query_first::<i64, String>(
                "SELECT max(LAST_INSERT_ID(note_id)) FROM notes".to_owned(),
            )?
            .unwrap())
    }

    pub fn find(filter: NotesOptions) -> mysql::Result<Option<Vec<Self>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        let query = format!(
            "select note_id from notes{};",
            if !filter.is_empty() {
                format!(" where {}", filter)
            } else {
                "".to_string()
            }
        );
        let ids: Vec<i64> = conn.query(query)?;

        // TODO (this and also in Workorders and Users)
        let notes = ids
            .iter()
            .map(|id| Note::by_id(*id))
            .filter(|note| note.is_ok() && note.as_ref().unwrap().is_some())
            .map(|note| note.unwrap().unwrap())
            .collect::<Vec<Note>>();
        if !notes.is_empty() {
            Ok(Some(notes))
        } else {
            Ok(None)
        }
    }

    pub fn by_id(id: i64) -> mysql::Result<Option<Self>> {
        let mut conn = crate::db::get_connection()?;
        if let Some(note_tuple) = conn.query_first::<NoteTuple, String>(format!(
            "select contents, user, posted, next_update from notes where note_id={}",
            id
        ))? {
            Ok(Some(Note::from(note_tuple)))
        } else {
            Ok(None)
        }
    }

    pub fn all_for_wo(wo_id: i64) -> mysql::Result<Option<Vec<Self>>> {
        let mut conn = crate::db::get_connection()?;
        let ids: Vec<i64> =
            conn.query::<i64, String>(format!("select note_id from notes where wo_key={}", wo_id))?;

        Ok(Some(
            ids.iter()
                .map(|id| Note::by_id(*id))
                .filter(|note| note.is_ok() && note.as_ref().unwrap().is_some())
                .map(|note| note.unwrap().unwrap())
                .collect(),
        ))
    }
}
