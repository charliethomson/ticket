use crate::db::models::*;
use mysql::{prelude::*, *};
use serde::Deserialize;
use std::collections::HashMap;

const FIELD_DELIM: &'static str = "#$++,";
const ITEM_DELIM: &'static str = "$!@;";
const TABLE_MARKER: &'static str = "$%^$**";

#[derive(Default, Deserialize)]
pub struct WorkorderFind {
    pub id: Option<i64>,
    pub origin: Option<i64>,
    pub travel_status: Option<String>,
    pub created: Option<i64>,
    pub quoted_time: Option<i64>,
    pub status: Option<String>,
    pub customer: Option<i64>,
    pub device: Option<i64>,
    pub brief: Option<String>,
}

impl WorkorderFind {
    pub fn into_delimited(&self) -> String {
        let mut items: HashMap<String, String> = HashMap::new();

        if let Some(id) = self.id {
            items.insert("id".to_owned(), id.to_string());
        }
        if let Some(origin) = self.origin {
            items.insert("origin".to_owned(), origin.to_string());
        }
        if let Some(travel_status) = &self.travel_status {
            items.insert("travel_status".to_owned(), travel_status.to_string());
        }
        if let Some(created) = &self.created {
            items.insert("created".to_owned(), created.to_string());
        }
        if let Some(quoted_time) = &self.quoted_time {
            items.insert("quoted".to_owned(), quoted_time.to_string());
        }
        if let Some(status) = &self.status {
            items.insert("status".to_owned(), status.to_string());
        }
        if let Some(customer) = self.customer {
            items.insert("customer".to_owned(), customer.to_string());
        }
        if let Some(device) = self.device {
            items.insert("device".to_owned(), device.to_string());
        }
        if let Some(brief) = &self.brief {
            items.insert("brief".to_owned(), brief.to_string());
        }

        let mut strs = vec![];

        for (key, value) in items {
            strs.push(format!("{}{}{}{:?}", TABLE_MARKER, key, FIELD_DELIM, value));
        }

        strs.join(ITEM_DELIM)
    }
    pub fn into_filter(&self) -> String {
        self.into_delimited()
            .replace(ITEM_DELIM, " and ")
            .replace(FIELD_DELIM, "=")
            .replace(TABLE_MARKER, "workorders.")
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
    pub fn into_delimited(&self) -> String {
        let mut items: HashMap<String, String> = HashMap::new();

        if let Some(id) = self.id {
            items.insert("id".to_owned(), id.to_string());
        }
        if let Some(serial) = &self.serial {
            items.insert("serial".to_owned(), serial.to_string());
        }
        if let Some(name) = &self.name {
            items.insert("name".to_owned(), name.to_string());
        }
        if let Some(password) = &self.password {
            items.insert("password".to_owned(), password.to_string());
        }

        let mut strs = vec![];

        for (key, value) in items {
            strs.push(format!("{}{}{}{:?}", TABLE_MARKER, key, FIELD_DELIM, value));
        }

        strs.join(ITEM_DELIM)
    }
    pub fn into_filter(&self) -> String {
        self.into_delimited()
            .replace(ITEM_DELIM, " and ")
            .replace(FIELD_DELIM, "=")
            .replace(TABLE_MARKER, "devices.")
    }
}

#[derive(Default, Deserialize)]
pub struct StoreOptions {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub contact_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}
impl StoreOptions {
    pub fn into_delimited(&self) -> String {
        let mut items: HashMap<String, String> = HashMap::new();

        if let Some(id) = self.id {
            items.insert("id".to_owned(), id.to_string());
        }
        if let Some(name) = &self.name {
            items.insert("store_name".to_owned(), name.to_string());
        }
        if let Some(contact_name) = &self.contact_name {
            items.insert("contact_name".to_owned(), contact_name.to_string());
        }
        if let Some(phone_number) = &self.phone_number {
            items.insert("phone_number".to_owned(), phone_number.to_string());
        }
        if let Some(email) = &self.email {
            items.insert("email_address".to_owned(), email.to_string());
        }
        if let Some(address) = &self.address {
            items.insert("address".to_owned(), address.to_string());
        }
        if let Some(city) = &self.city {
            items.insert("city".to_owned(), city.to_string());
        }
        if let Some(state) = &self.state {
            items.insert("state".to_owned(), state.to_string());
        }
        if let Some(zip) = &self.zip {
            items.insert("zip".to_owned(), zip.to_string());
        }

        let mut strs = vec![];

        for (key, value) in items {
            strs.push(format!("{}{}{}{:?}", TABLE_MARKER, key, FIELD_DELIM, value));
        }

        strs.join(ITEM_DELIM)
    }
    pub fn into_filter(&self) -> String {
        self.into_delimited()
            .replace(ITEM_DELIM, " and ")
            .replace(FIELD_DELIM, "=")
            .replace(TABLE_MARKER, "stores.")
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
    pub fn into_delimited(&self) -> String {
        let mut items: HashMap<String, String> = HashMap::new();

        if let Some(id) = self.id {
            items.insert("id".into(), id.to_string());
        }
        if let Some(name) = &self.name {
            items.insert("name".into(), name.to_string());
        }
        if let Some(phone_number) = &self.phone_number {
            items.insert("phone_number".into(), phone_number.to_string());
        }
        if let Some(email) = &self.email {
            items.insert("email".into(), email.to_string());
        }
        if let Some(store_id) = &self.store_id {
            items.insert("store_id".into(), store_id.to_string());
        }

        let mut strs = vec![];

        for (key, value) in items {
            strs.push(format!("{}{}{}{:?}", TABLE_MARKER, key, FIELD_DELIM, value));
        }

        strs.join(ITEM_DELIM)
    }
    pub fn into_filter(&self) -> String {
        self.into_delimited()
            .replace(ITEM_DELIM, " and ")
            .replace(FIELD_DELIM, "=")
            .replace(TABLE_MARKER, "customers.")
    }
}

#[derive(Default, Deserialize)]
pub struct UserFind {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
}
impl UserFind {
    pub fn into_delimited(&self) -> String {
        let mut items: HashMap<String, String> = HashMap::new();

        if let Some(id) = self.id {
            items.insert("id".to_owned(), id.to_string());
        }
        if let Some(name) = &self.name {
            items.insert("name".to_owned(), name.to_string());
        }
        if let Some(phone_number) = &self.phone_number {
            items.insert("phone_number".to_owned(), phone_number.to_string());
        }

        let mut strs = vec![];

        for (key, value) in items {
            strs.push(format!("{}{}{}{:?}", TABLE_MARKER, key, FIELD_DELIM, value));
        }

        strs.join(ITEM_DELIM)
    }
    pub fn into_filter(&self) -> String {
        self.into_delimited()
            .replace(ITEM_DELIM, " and ")
            .replace(FIELD_DELIM, "=")
            .replace(TABLE_MARKER, "users.")
    }
}

#[derive(Default, Deserialize)]
pub struct NotesOptions {
    pub note_id: Option<i64>,
    pub workorder_id: Option<i64>,
}
impl NotesOptions {
    pub fn into_delimited(&self) -> String {
        let mut items: HashMap<String, String> = HashMap::new();

        if let Some(note_id) = self.note_id {
            items.insert("note_id".to_owned(), note_id.to_string());
        }
        if let Some(workorder_id) = self.workorder_id {
            items.insert("wo_key".to_owned(), workorder_id.to_string());
        }

        let mut strs = vec![];

        for (key, value) in items {
            strs.push(format!("{}{}{}{:?}", TABLE_MARKER, key, FIELD_DELIM, value));
        }

        strs.join(ITEM_DELIM)
    }
    pub fn into_filter(&self) -> String {
        self.into_delimited()
            .replace(ITEM_DELIM, " and ")
            .replace(FIELD_DELIM, "=")
            .replace(TABLE_MARKER, "notes.")
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

    pub fn find(_filter: DeviceFind) -> mysql::Result<Option<Self>> {
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
            .query_first::<i64, String>("SELECT LAST_INSERT_ID(id) FROM stores".to_owned())?
            .unwrap())
    }

    pub fn find(filter: StoreOptions) -> mysql::Result<Option<Vec<Self>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        let stores: Vec<Store> = conn
            .query::<i64, String>(format!(
                "select id from stores{}",
                if filter.len() != 0 {
                    format!(" where {}", filter)
                } else {
                    "".to_string()
                }
            ))?
            .iter()
            // BIG TODO
            .map(|id| Store::by_id(*id))
            .filter(|res| res.is_ok() && res.as_ref().unwrap().is_some())
            .map(|resop| resop.unwrap().unwrap())
            .collect();

        Ok(if stores.len() == 0 {
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
        Ok(0)
    }

    pub fn find(_filter: CustomerFind) -> mysql::Result<Option<Self>> {
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

    pub fn find(filter: UserFind) -> mysql::Result<Option<Vec<Self>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        let query = format!(
            "select id from users{};",
            if filter.len() != 0 {
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
        if users.len() != 0 {
            Ok(Some(users))
        } else {
            Ok(None)
        }
    }

    pub fn update(&mut self, changes: UserFind) -> mysql::Result<bool> {
        let mut conn = crate::db::get_connection()?;

        let rawstr = changes.into_delimited();
        let update_str = format!(
            "set {}",
            rawstr
                .replace(ITEM_DELIM, ", ")
                .replace(FIELD_DELIM, "=")
                .replace(TABLE_MARKER, "")
        );

        dbg!(update_str.clone());

        conn.query::<Vec<_>, String>(format!(
            "update users {} where users.id={}",
            update_str, self.id
        ))?;

        Ok(true)
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

    pub fn find(_filter: NotesOptions) -> mysql::Result<Option<Self>> {
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
