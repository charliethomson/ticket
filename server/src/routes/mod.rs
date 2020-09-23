pub mod devices;
pub mod notes;
pub mod stores;
pub mod users;
pub mod workorders;

pub use devices::*;
pub use notes::*;
pub use stores::*;
pub use users::*;
pub use workorders::*;

use serde::Serialize;

#[derive(Serialize)]
pub struct OkMessage<Message: Serialize> {
    pub ok: bool,
    pub message: Option<Message>,
}
