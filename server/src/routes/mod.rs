mod api;
pub use api::*;

mod front_end;
pub use front_end::*;

mod auth;
pub use auth::*;

#[derive(Debug)]
pub struct AppState {
    pub oauth_client: oauth2::basic::BasicClient,
    pub verifier: oauth2::PkceCodeVerifier,
    pub csrf: oauth2::CsrfToken,
    pub auth_url: String,
}
