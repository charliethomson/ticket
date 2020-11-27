pub mod customers;
pub mod devices;
pub mod notes;
pub mod stores;
pub mod users;
pub mod workorders;

pub use customers::*;
pub use devices::*;
pub use notes::*;
pub use stores::*;
pub use users::*;
pub use workorders::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Limit {
    limit: Option<i64>,
}
impl From<Limit> for i64 {
    fn from(l: Limit) -> i64 {
        l.limit.unwrap_or(10)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::routes) struct OkMessage<Message> {
    pub ok: bool,
    pub message: Option<Message>,
}
