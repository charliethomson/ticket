mod db;
mod routes;

use actix_session::CookieSession;
use actix_web::{App, HttpServer};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};

const URL: &str = "localhost:8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use routes::*;
    let server = HttpServer::new(|| {
        let google_client_id = ClientId::new(
            "504338965859-keccmcqnnf6rjf0dhog9suld0l8qtecu.apps.googleusercontent.com".to_owned(),
        );
        let google_client_secret = ClientSecret::new("cx7z1cia757gtVyLGg6fEW81".to_owned());
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
            .expect("Invalid token endpoint URL");

        // Set up the config for the Google OAuth2 process.
        let oauth_client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_url(
            RedirectUrl::new(format!("http://{}/api/auth/response", URL))
                .expect("Invalid redirect URL"),
        );

        let (code_challenge, verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, _) = oauth_client
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
            .set_pkce_challenge(code_challenge)
            .url();

        App::new()
            .data(AppState {
                oauth_client,
                verifier,
                auth_url: auth_url.to_string(),
            })
            .wrap(CookieSession::signed(&[0; 32]))
            .service(users_post)
            .service(users_get)
            .service(users_put)
            //
            .service(workorders_post)
            .service(workorders_get)
            .service(workorders_put)
            //
            .service(stores_put)
            .service(stores_post)
            .service(stores_get)
            //
            .service(notes_get)
            .service(notes_post)
            //
            .service(devices_put)
            .service(devices_post)
            .service(devices_get)
            //
            .service(customers_put)
            .service(customers_post)
            .service(customers_get)
            .service(auth_login)
            .service(auth_response)
    })
    .bind(URL)?;
    println!("Listening on http://{}", URL);

    server.run().await
}
