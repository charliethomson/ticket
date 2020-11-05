use crate::db::{
    types::{
        Customer, Device, Note, NoteResponse, Store, Workorder, WorkorderOptions,
        WorkorderResponse, WorkorderTuple,
    },
    Options,
};
use mysql::prelude::Queryable;
use std::convert::TryFrom;

impl Workorder {
    pub fn find(filter: WorkorderOptions) -> mysql::Result<Vec<WorkorderResponse>> {
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
