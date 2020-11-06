use crate::db::{
    types::{Store, StoreOptions, StoreTuple},
    Options,
};
use mysql::prelude::Queryable;

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
