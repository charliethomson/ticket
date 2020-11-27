mod api;
pub use api::*;

mod front_end;
pub use front_end::*;

// mod auth;
// pub use auth::*;

use {
    chrono::{DateTime, Utc},
    oauth2::{
        basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
        RedirectUrl, RefreshToken, Scope, TokenUrl,
    },
    std::env,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub oauth_client: BasicClient,
    // pub verifier: oauth2::PkceCodeVerifier,
    pub csrf: CsrfToken,
    pub auth_url: String,
    pub expiry: DateTime<Utc>,
    pub refresh_token: RefreshToken,
}
impl AppState {
    pub fn new() -> Self {
        let google_client_id = ClientId::new(
            env::var("GOOGLE_CLIENT_ID")
                .expect("Missing the GOOGLE_CLIENT_ID environment variable."),
        );
        let google_client_secret = ClientSecret::new(
            env::var("GOOGLE_CLIENT_SECRET")
                .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
        );
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
            .expect("Invalid token endpoint URL");

        let client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_url(
            RedirectUrl::new("http://localhost:8080/api/auth/response".to_string())
                .expect("Invalid redirect URL"),
        );

        let (_pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/user.organization.read".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/user.phonenumbers.read".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            // TODO: Verifier p/ty
            // .set_pkce_challenge(pkce_code_challenge)
            .url();

        AppState {
            oauth_client: client,
            // verifier: pkce_code_verifier,
            csrf: csrf_state,
            auth_url: authorize_url.to_string(),
            expiry: chrono::Utc::now(),
            refresh_token: oauth2::RefreshToken::new("".to_owned()),
        }
    }
}
