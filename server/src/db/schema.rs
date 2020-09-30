use crate::db::models::*;
use chrono::prelude::*;
use mysql::{prelude::*, *};
use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct WorkorderFind {
    pub id: Option<i64>,
    pub origin: Option<i64>,
    pub travel_status: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub quoted_time: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub customer: Option<i64>,
    pub device: Option<i64>,
    pub brief: Option<String>,
}
impl WorkorderFind {
    pub fn into_filter(&self) -> String {
        let mut filter = String::new();
        let mut pushed = false;

        if let Some(id) = self.id {
            filter.push_str(&format!("workorders.id={}", id));
            pushed = true;
        }
        if let Some(origin) = self.origin {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("workorders.origin={}", origin));
            pushed = true;
        }
        if let Some(travel_status) = &self.travel_status {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("workorders.travel_status=\"{}\"", travel_status));
            pushed = true;
        }
        if let Some(created) = &self.created {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("workorders.created=\"{}\"", created));
            pushed = true;
        }
        if let Some(quoted_time) = &self.quoted_time {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("workorders.quoted=\"{}\"", quoted_time));
            pushed = true;
        }
        if let Some(status) = &self.status {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("workorders.status=\"{}\"", status));
            pushed = true;
        }
        if let Some(customer) = self.customer {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("workorders.customer={}", customer));
            pushed = true;
        }
        if let Some(device) = self.device {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("workorders.device={}", device));
            pushed = true;
        }
        if let Some(brief) = &self.brief {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("workorders.brief=\"{}\"", brief));
        }
        if pushed {
            filter
        } else {
            String::new()
        }
    }
}

#[derive(Default)]
pub struct DeviceFind {
    pub id: Option<i64>,
    pub serial: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
}
impl DeviceFind {
    pub fn into_filter(&self) -> String {
        let mut filter = String::new();
        let mut pushed = false;

        if let Some(id) = self.id {
            filter.push_str(&format!("devices.id={}", id));
            pushed = true;
        }
        if let Some(serial) = &self.serial {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("devices.serial=\"{}\"", serial));
            pushed = true;
        }
        if let Some(name) = &self.name {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("devices.name=\"{}\"", name));
            pushed = true;
        }
        if let Some(password) = &self.password {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("devices.password=\"{}\"", password));
        }
        if pushed {
            filter
        } else {
            String::new()
        }
    }
}

#[derive(Default)]
pub struct StoreFind {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub contact_name: Option<String>,
    pub phone_number: Option<i32>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<i32>,
}
impl StoreFind {
    pub fn into_filter(&self) -> String {
        let mut filter = String::new();
        let mut pushed = false;

        if let Some(id) = self.id {
            filter.push_str(&format!("stores.id={}", id));
            pushed = true;
        }
        if let Some(name) = &self.name {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("stores.name=\"{}\"", name));
            pushed = true;
        }
        if let Some(contact_name) = &self.contact_name {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("stores.contact_name=\"{}\"", contact_name));
            pushed = true;
        }
        if let Some(phone_number) = &self.phone_number {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("stores.phone_number=\"{}\"", phone_number));
            pushed = true;
        }
        if let Some(email) = &self.email {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("stores.email_address=\"{}\"", email));
            pushed = true;
        }
        if let Some(address) = &self.address {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("stores.address=\"{}\"", address));
            pushed = true;
        }
        if let Some(city) = &self.city {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("stores.city=\"{}\"", city));
            pushed = true;
        }
        if let Some(state) = &self.state {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("stores.state=\"{}\"", state));
            pushed = true;
        }
        if let Some(zip) = &self.zip {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("stores.zip={}", zip));
        }
        if pushed {
            filter
        } else {
            String::new()
        }
    }
}

#[derive(Default)]
pub struct CustomerFind {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub store_id: Option<i64>,
}
impl CustomerFind {
    pub fn into_filter(&self) -> String {
        let mut filter = String::new();
        let mut pushed = false;

        if let Some(id) = self.id {
            filter.push_str(&format!("customers.id={}", id));
            pushed = true;
        }
        if let Some(name) = &self.name {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("customers.name=\"{}\"", name));
            pushed = true;
        }
        if let Some(phone_number) = &self.phone_number {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("customers.phone_number=\"{}\"", phone_number));
            pushed = true;
        }
        if let Some(email) = &self.email {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("customers.email=\"{}\"", email));
            pushed = true;
        }
        if let Some(store_id) = &self.store_id {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("customers.store_id={}", store_id));
        }
        if pushed {
            filter
        } else {
            String::new()
        }
    }
}

#[derive(Default)]
pub struct UserFind {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    // TODO:
}
impl UserFind {
    // Converts the struct into a string:
    // { id: None, name: Some("foo"), phone_number: Some("5402292296") }
    // -> "where users.name="foo" and users.phone_number="5402292296""
    pub fn into_filter(&self) -> String {
        let mut filter = String::new();
        let mut pushed = false;

        if let Some(id) = self.id {
            filter.push_str(&format!("users.id={}", id));
            pushed = true;
        }
        if let Some(name) = &self.name {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("users.name=\"{}\"", name));
            pushed = true;
        }
        if let Some(phone_number) = &self.phone_number {
            if pushed {
                filter.push_str(" and ")
            }
            filter.push_str(&format!("users.phone_number=\"{}\"", phone_number));
        }
        if pushed {
            filter
        } else {
            String::new()
        }
    }
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
        Ok(conn
            .query_first::<i64, String>("SELECT LAST_INSERT_ID(id) FROM workorders".to_owned())?)
    }

    pub fn find(filter: WorkorderFind) -> mysql::Result<Option<Vec<WorkorderResponse>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        let query = format!(
            "select id from workorders{};",
            if filter.len() != 0 {
                format!(" where {}", filter)
            } else {
                "".to_string()
            }
        );
        eprintln!("{}", query);
        let ids: Vec<i64> = conn.query(query)?;

        let wos = ids
            .iter()
            .map(|id| Workorder::by_id(*id))
            .filter(|wo| wo.is_ok() && wo.as_ref().unwrap().is_some())
            .map(|wo| wo.unwrap().unwrap())
            .collect::<Vec<WorkorderResponse>>();
        if wos.len() != 0 {
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
                created: created,
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

impl Device {
    pub fn insert(&self) -> mysql::Result<i64> {
        Ok(0)
    }

    pub fn find(filter: DeviceFind) -> mysql::Result<Option<Self>> {
        Ok(None)
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
        Ok(0)
    }

    pub fn find(filter: StoreFind) -> mysql::Result<Option<Self>> {
        Ok(None)
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
        Ok(0)
    }

    pub fn find(filter: CustomerFind) -> mysql::Result<Option<Self>> {
        Ok(None)
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
    pub fn insert(&self) -> mysql::Result<i64> {
        Ok(0)
    }

    pub fn find(filter: UserFind) -> mysql::Result<Option<Self>> {
        Ok(None)
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

        // TODO: Unwrap
        Ok(conn
            .query_first::<i64, String>("SELECT LAST_INSERT_ID(note_id) FROM notes".to_owned())?
            .unwrap())
    }

    pub fn find(filter: StoreFind) -> mysql::Result<Option<Self>> {
        Ok(None)
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
