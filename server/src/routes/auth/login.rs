use crate::{
    db::{User, UserResponse},
    routes::{AppState, OkMessage, UserNew},
};
use actix_identity::Identity;
use actix_web::{
    client::{Client, Connector},
    get, web, HttpResponse,
};
use oauth2::{reqwest::http_client, AuthorizationCode, /*CsrfToken,*/ TokenResponse};
use openssl::ssl::{SslConnector, SslMethod};

#[get("/api/auth/login")]
pub async fn auth_login(identity: Identity, data: web::Data<AppState>) -> HttpResponse {
    match identity.identity() {
        Some(_) => HttpResponse::Ok()
            .header(actix_web::http::header::LOCATION, "/")
            .json(OkMessage {
                ok: true,
                message: Some("User already logged in"),
            }),
        _ => HttpResponse::Found()
            .header(actix_web::http::header::LOCATION, data.auth_url.to_string())
            .finish(),
    }
}

#[get("/api/auth/response")]
pub async fn auth_response(
    identity: Identity,
    data: web::Data<AppState>,
    params: web::Query<super::AuthRequest>,
) -> HttpResponse {
    // Get the authorization code from the request query
    let code = AuthorizationCode::new(params.code.clone());

    // TODO: verify state
    // let state = CsrfToken::new(params.state.to_owned());

    // Get a mutable handle to the app state
    let mut data = data.into_inner();
    let mut data = std::sync::Arc::make_mut(&mut data);

    // Get an access token from the oauth client
    let token = data
        .oauth_client
        .exchange_code(code)
        // TODO: Verifier p/ty
        // .set_pkce_verifier(PkceCodeVerifier::new(data.verifier.secret().clone()))
        .request(http_client)
        .expect("Failed to get access token");

    // TODO
    // data.refresh_token = oauth2::RefreshToken::new(
    //     token
    //         .refresh_token()
    //         .map(|tok| tok.secret().to_string())
    //         .unwrap_or_default(),
    // );

    // Set the expiration time, which we get from the access token.
    // This will help us refresh our access token
    data.expiry = match token
        .expires_in()
        .map(|dur| chrono::Duration::from_std(dur).ok())
        .flatten()
    {
        Some(expiry) => chrono::Utc::now() + expiry,
        None => {
            return HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some("Failed to set token expiration"),
            })
        }
    };

    // Make an SSL capable request client
    let client = Client::builder()
        .connector(
            Connector::new()
                .ssl(SslConnector::builder(SslMethod::tls()).unwrap().build())
                .finish(),
        )
        .finish();

    // Request the user's info from google's API
    match client
        .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<crate::routes::auth::UserInfo>()
        .await
    {
        // If we got a valid response from the API
        Ok(response) => {
            match response.hd.as_ref() {
                // Require that the user signed in with a ubif email address
                Some(hd) if hd == "ubreakifix.com" => {
                    // Insert or request the user id associated with the user information
                    // we got from google
                    let response =
                        crate::routes::auth::create_new_user_internal(UserNew::from(response))
                            .await;

                    if response.ok {
                        // Get the ID returned from our api, add it to the session storage,
                        // and build a response redirecting to the index with the body we were returned
                        if let Some(id) = &response.message {
                            identity.remember(id.to_string());
                        } else {
                            // This should never happen
                            unreachable!()
                        }
                        HttpResponse::Found()
                            .header(actix_web::http::header::LOCATION, "/")
                            .json(response)
                    } else {
                        HttpResponse::InternalServerError().json(response)
                    }
                }
                _ => {
                    // TODO
                    HttpResponse::Unauthorized()
                        .body("Unable to login with that email :(".to_string())
                }
            }
        }
        Err(e) => {
            // TODO Status Code
            HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some(format!("3: {}", e.to_string())),
            })
        }
    }
}

// #[get("/api/auth/refresh")]
// pub async fn auth_refresh(identity: Identity, data: web::Data<AppState>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

#[get("/api/auth/me")]
pub async fn auth_me(identity: Identity) -> HttpResponse {
    // get the user id from the session
    match identity.identity() {
        // See if the user id is valid
        // TODO: Unwrap should be safe
        Some(id) => match User::by_id(id.parse::<i64>().unwrap()) {
            // if it is, return an ok response with the user
            Ok(Some(user)) => HttpResponse::Ok().json(OkMessage {
                ok: true,
                message: Some(UserResponse::from(user)),
            }),
            // Otherwise, return not found
            Ok(None) => HttpResponse::NotFound().json(OkMessage::<()> {
                ok: true,
                message: None,
            }),
            // If we had an error, let teh user know :)
            Err(e) => HttpResponse::NotFound().json(OkMessage {
                ok: false,
                message: Some(e.to_string()),
            }),
        },
        // self explanatory
        _ => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: Some("No user logged in"),
        }),
    }
}
