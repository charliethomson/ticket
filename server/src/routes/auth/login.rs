use crate::{
    db::{User, UserResponse},
    routes::{AppState, OkMessage, UserNew},
};
use actix_session::Session;
use actix_web::{
    client::{Client, Connector},
    dev::HttpResponseBuilder,
    get, web, HttpResponse,
};
use oauth2::{reqwest::http_client, AuthorizationCode, /*CsrfToken,*/ TokenResponse};
use openssl::ssl::{SslConnector, SslMethod};

#[get("/api/auth/login")]
pub async fn auth_login(session: Session, data: web::Data<AppState>) -> HttpResponse {
    match session.get("authenticated") {
        // User not logged in
        Ok(Some(false)) | Ok(None) => HttpResponse::Found()
            .header(actix_web::http::header::LOCATION, data.auth_url.to_string())
            .finish(),
        // User already logged in
        // FIXME: Not redirecting
        Ok(Some(true)) => HttpResponse::Ok()
            .header(actix_web::http::header::LOCATION, "/")
            .json(OkMessage {
                ok: true,
                message: Some("User already logged in"),
            }),
        // Session error
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

#[get("/api/auth/response")]
pub async fn auth_response(
    session: Session,
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
            // Set the authenticated flag in the session
            session
                .set("authenticated", true)
                .expect("FAILED TO SET SESSION KEY authenticated");
            match response.hd.as_ref() {
                // Require that the user signed in with a ubif email address
                Some(hd) if hd == "ubreakifix.com" => {
                    // Insert or request the user id associated with the user information
                    // we got from google
                    match client
                        .post(format!("http://{}/api/users", crate::URL))
                        .send_json(&UserNew::from(response))
                        .await
                    {
                        // If we got a good response from our api, attempt to parse the response as an OkMessage
                        Ok(mut res) => match res.json::<OkMessage<i64>>().await {
                            Ok(body) => {
                                // Get the ID returned from our api, add it to the session storage,
                                // and build a response redirecting to the index with the body we were returned
                                if let Some(id) = body.message {
                                    println!("\n\n\nSetting user_id to {}\n\n\n", id);
                                    session.set("userId", id).unwrap();
                                    println!("Successfully authenticated user {}", id);
                                } else {
                                    // This should never happen
                                    unreachable!()
                                }
                                HttpResponse::Found()
                                    .header(actix_web::http::header::LOCATION, "/")
                                    .json(body)
                            }
                            Err(e) => HttpResponseBuilder::new(res.status()).json(OkMessage {
                                ok: false,
                                message: Some(e.to_string()),
                            }),
                        },
                        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
                            ok: false,
                            message: Some(e.to_string()),
                        }),
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
                message: Some(e.to_string()),
            })
        }
    }
}

// #[get("/api/auth/refresh")]
// pub async fn auth_refresh(session: Session, data: web::Data<AppState>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

#[get("/api/auth/me")]
pub async fn auth_me(session: Session) -> HttpResponse {
    // get the user id from the session
    match session.get("userId") {
        // See if the user id is valid
        Ok(Some(id)) => match User::by_id(id) {
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
        _ => HttpResponse::Unauthorized().json(OkMessage {
            ok: false,
            message: Some("No user logged in"),
        }),
    }
}
