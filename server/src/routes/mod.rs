mod api;
pub use api::*;

mod auth;
pub use auth::*;

mod r#static;
pub use r#static::*;

#[derive(Debug)]
pub struct AppState {
    pub oauth_client: oauth2::basic::BasicClient,
    pub verifier: oauth2::PkceCodeVerifier,
    pub auth_url: String,
}
