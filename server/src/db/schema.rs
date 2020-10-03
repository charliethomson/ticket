#![allow(dead_code)]

// TODO: Convert `update` to a proc macro

use crate::db::models::*;
use crate::routes::users::UserNew;
use mysql::{prelude::*, *};
use schema_proc_macros::*;
use serde::Deserialize;

const FIELD_DELIM: &str = "#$++,";
const ITEM_DELIM: &str = "$!@;";
const TABLE_MARKER: &str = "$%^$#$!$@#";
const PADDING_VALUE: &str = "$%&&#$*@@";

pub trait Options {
    fn into_delimited(&self) -> String;
    fn into_filter(&self) -> String;
    fn into_update(&self) -> String;
}

pub trait Update<Changes: Options> {
    fn update(&mut self, changes: Changes) -> mysql::Result<()> {
        let mut conn = crate::db::get_connection()?;
        let query = changes.into_update();
        conn.query::<Vec<_>, String>(query)?;
        Ok(())
    }
}

#[derive(Default, Deserialize, Debug, Clone, Options)]
pub struct WorkorderOptions {
    pub id: Option<i64>,
    pub origin: Option<i64>,
    pub travel_status: Option<String>,
    pub created: Option<i64>,
    #[db_name("quoted")]
    pub quoted_time: Option<i64>,
    #[db_name("workorder_status")]
    pub status: Option<String>,
    pub customer: Option<i64>,
    pub device: Option<i64>,
    pub brief: Option<String>,
}

#[derive(Default, Deserialize, Options)]
pub struct DeviceOptions {
    pub id: Option<i64>,
    #[db_name("serial_no")]
    pub serial: Option<String>,
    #[db_name("device_name")]
    pub name: Option<String>,
    pub password: Option<String>,
}

#[derive(Default, Deserialize, Options)]
pub struct StoreOptions {
    pub id: Option<i64>,
    #[db_name("store_name")]
    pub name: Option<String>,
    pub contact_name: Option<String>,
    pub phone_number: Option<String>,
    #[db_name("email_address")]
    pub email: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}

#[derive(Default, Deserialize, Options)]
pub struct CustomerOptions {
    pub id: Option<i64>,
    #[db_name("customer_name")]
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub store_id: Option<i64>,
}

#[derive(Default, Deserialize, Options)]
pub struct UserOptions {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Default, Deserialize, Options)]
pub struct NotesOptions {
    #[db_name("note_id")]
    pub id: Option<i64>,
    #[db_name("wo_key")]
    pub workorder_id: Option<i64>,
}

impl Workorder {
    pub fn insert(&self) -> mysql::Result<Option<i64>> {
        let mut conn = crate::db::get_connection()?;
        conn.exec_drop(
            "insert into workorders
        (   origin, 
            travel_status, 
            created, 
            quoted, 
            workorder_status, 
            customer, 
            device,
            brief
        ) values (
            :origin, 
            :travel_status, 
            :created, 
            :quoted, 
            :workorder_status, 
            :customer, 
            :device,
            :brief
        );",
            params! {
                "origin" =>self.origin,
                "travel_status" =>self.travel_status.clone(),
                "created" => self.created.timestamp(),
                "quoted" => self.quoted_time.map(|dt| dt.timestamp()),
                "workorder_status" =>self.status.clone(),
                "customer" =>self.customer,
                "device" =>self.device,
                "brief" =>self.brief.clone(),
            },
        )?;
        Ok(conn.query_first::<i64, String>(
            "SELECT max(LAST_INSERT_ID(id)) FROM workorders".to_owned(),
        )?)
    }

    pub fn find(filter: WorkorderOptions) -> mysql::Result<Option<Vec<WorkorderResponse>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
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
            .filter(|wo| wo.is_ok() && wo.as_ref().unwrap().is_some())
            .map(|wo| wo.unwrap().unwrap())
            .collect::<Vec<WorkorderResponse>>();
        if !wos.is_empty() {
            Ok(Some(wos))
        } else {
            Ok(None)
        }
    }

    pub fn by_id(id: i64) -> mysql::Result<Option<WorkorderResponse>> {
        let mut conn = crate::db::get_connection()?;
        if let Some((
            id,
            store_id,
            travel_status,
            created,
            quoted,
            status,
            device_id,
            customer_id,
            brief,
        )) = conn
            .query_first::<(i64, i64, String, i64, Option<i64>, String, i64, i64, String), String>(
                format!("select * from workorders where workorders.id={}", id),
            )?
        {
            let origin = match Store::by_id(store_id)? {
                Some(store) => store,
                _ => panic!(),
            };
            let device = match Device::by_id(device_id)? {
                Some(device) => device,
                _ => panic!(),
            };
            let customer = match Customer::by_id(customer_id)? {
                Some(customer) => customer,
                _ => panic!(),
            };
            let notes = match Note::all_for_wo(id)? {
                Some(notes) => notes,
                _ => panic!(),
            };

            Ok(Some(WorkorderResponse {
                workorder_id: id,
                origin,
                travel_status,
                created,
                quoted_time: quoted,
                status,
                customer,
                device,
                brief,
                notes,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Update<WorkorderOptions> for WorkorderResponse {}
impl Update<DeviceOptions> for Device {}
impl Update<StoreOptions> for Store {}
impl Update<CustomerOptions> for Customer {}
impl Update<UserOptions> for User {}

impl Device {
    pub fn insert(&self) -> mysql::Result<i64> {
        let mut conn = crate::db::get_connection()?;
        conn.exec_drop(
            "insert into devices
        (   
            serial_no,
            device_name,
            customer,
            password
        ) values (
            :serial_no,
            :device_name,
            :customer,
            :password
        );",
            params! {
                "serial_no" => self.serial.clone(),
                "device_name" => self.name.clone(),
                "customer" => self.customer_id,
                "password" => self.password.clone()
            },
        )?;

        Ok(conn
            .query_first::<i64, String>("SELECT max(LAST_INSERT_ID(id)) FROM devices".to_owned())?
            .unwrap())
    }

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
    pub fn insert(&self) -> mysql::Result<i64> {
        let mut conn = crate::db::get_connection()?;
        conn.exec_drop(
            "insert into stores
        (
            store_name,
            contact_name,
            phone_number,
            email_address,
            address,
            city,
            state,
            zip
        ) values (
            :store_name,
            :contact_name,
            :phone_number,
            :email_address,
            :address,
            :city,
            :state,
            :zip
        );",
            params! {
                "store_name" => self.name.to_string(),
                "contact_name" => self.contact_name.to_string(),
                "phone_number" => self.phone_number.to_string(),
                "email_address" => self.email.to_string(),
                "address" => self.address.to_string(),
                "city" => self.city.to_string(),
                "state" => self.state.to_string(),
                "zip" => self.zip.to_string()
            },
        )?;

        // Unwrap _should_ be safe because LAST_INSERT_ID would be set by the query above.
        // FIXME: I'll keep this here just in case, but it should be fine.
        Ok(conn
            .query_first::<i64, String>("SELECT max(LAST_INSERT_ID(id)) FROM stores".to_owned())?
            .unwrap())
    }

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
    pub fn insert(&self) -> mysql::Result<i64> {
        let mut conn = crate::db::get_connection()?;
        conn.exec_drop(
            "insert into customers
        (   
            customer_name,
            phone_number,
            email_address,
            store_id
        ) values (
            :customer_name,
            :phone_number,
            :email_address,
            :store_id
        );",
            params! {
                "customer_name" => self.name.clone(),
                "phone_number" => self.phone_number.clone(),
                "email_address" => self.email.clone(),
                "store_id" => self.store_id,
            },
        )?;

        // Unwrap _should_ be safe because LAST_INSERT_ID would be set by the query above.
        // FIXME: I'll keep this here just in case, but it should be fine.
        Ok(conn
            .query_first::<i64, String>("SELECT max(LAST_INSERT_ID(id)) FROM customers".to_owned())?
            .unwrap())
    }

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
        conn.exec_drop(
            "insert into users
        (
            name,
            phone_number
        ) values (
            :name,
            :phone_number
        );",
            params! {
                "name" => user.name.clone(),
                "phone_number" => user.phone_number
            },
        )?;

        Ok(conn
            .query_first::<i64, String>("SELECT max(LAST_INSERT_ID(id)) FROM users".to_owned())?
            .unwrap())
    }

    pub fn find(filter: UserOptions) -> mysql::Result<Option<Vec<Self>>> {
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
        eprintln!("{}", query);
        let ids: Vec<i64> = conn.query(query)?;

        // TODO (this and also in Workorders)
        let users = ids
            .iter()
            .map(|id| User::by_id(*id))
            .filter(|user| user.is_ok() && user.as_ref().unwrap().is_some())
            .map(|user| user.unwrap().unwrap())
            .collect::<Vec<User>>();
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
                "posted" => self.created.timestamp(),
                "next_update" => self.next_update.map(|op| op.timestamp())
            },
        )?;

        Ok(conn
            .query_first::<i64, String>("SELECT LAST_INSERT_ID(note_id) FROM notes".to_owned())?
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
