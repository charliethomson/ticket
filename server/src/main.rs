mod db;
mod routes;

use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{middleware::Logger, App, HttpServer};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};
use std::env;

const URL: &str = "localhost:8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    use routes::*;
    let server = HttpServer::new(|| {
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

        let (_pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

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

        App::new()
            .data(AppState {
                oauth_client: client,
                verifier: pkce_code_verifier,
                csrf: csrf_state,
                auth_url: authorize_url.to_string(),
            })
            .wrap(CookieSession::signed(&[0; 32]).name("offsite").secure(true))
            .wrap(Cors::new().send_wildcard().finish())
            .wrap(Logger::default())
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
            //
            .service(auth_login)
            .service(auth_response)
            //
            .service(front_end)
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind(URL)?;
    println!("Listening on http://{}", URL);

    server.run().await
}
