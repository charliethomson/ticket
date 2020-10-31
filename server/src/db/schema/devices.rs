use crate::db::{
    types::{Device, DeviceOptions, DeviceTuple},
    Options,
};
use mysql::prelude::Queryable;

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
