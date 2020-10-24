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

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::routes) struct OkMessage<Message> {
    pub ok: bool,
    pub message: Option<Message>,
}
