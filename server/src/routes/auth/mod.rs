mod login;
pub use login::*;

#[derive(serde::Deserialize, Debug)]
pub struct AuthRequest {
    code: String,
    state: String,
    scope: String,
}
