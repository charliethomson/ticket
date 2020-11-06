use crate::db::{
    types::{Customer, CustomerOptions, CustomerTuple},
    Options,
};
use mysql::prelude::Queryable;

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
