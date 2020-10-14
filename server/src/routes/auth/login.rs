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
pub async fn auth_login(data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, data.auth_url.to_string())
        .finish()
}

#[get("/api/auth/response")]
pub async fn auth_response(
    session: Session,
    data: web::Data<AppState>,
    params: web::Query<super::AuthRequest>,
) -> HttpResponse {
    let code = AuthorizationCode::new(params.code.clone());
    // TODO: verify state
    // let state = CsrfToken::new(params.state.to_owned());
    let mut data = data.into_inner();
    println!("{:#?}", data);
    let mut data = std::sync::Arc::make_mut(&mut data);
    let token = data
        .oauth_client
        .exchange_code(code)
        // TODO: Verifier p/ty
        // .set_pkce_verifier(PkceCodeVerifier::new(data.verifier.secret().clone()))
        .request(http_client)
        .expect("Failed to get access token");
    println!("TOKEN: {:#?}", token);

    // TODO
    // data.refresh_token = oauth2::RefreshToken::new(
    //     token
    //         .refresh_token()
    //         .map(|tok| tok.secret().to_string())
    //         .unwrap_or_default(),
    // );

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

    println!("{:#?}", data);
    let client = Client::builder()
        .connector(
            Connector::new()
                .ssl(SslConnector::builder(SslMethod::tls()).unwrap().build())
                .finish(),
        )
        .finish();
    match client
        .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<crate::routes::auth::UserInfo>()
        .await
    {
        Ok(response) => {
            session
                .set("authenticated", true)
                .expect("FAILED TO SET SESSION KEY authenticated");
            match response.hd.as_ref() {
                Some(hd) if hd == "ubreakifix.com" => {
                    match client
                        .post(format!("http://{}/api/users", crate::URL))
                        .send_json(&UserNew::from(response))
                        .await
                    {
                        Ok(mut res) => match res.json::<OkMessage<i64>>().await {
                            Ok(body) => {
                                if let Some(id) = body.message {
                                    session.set("userId", id).unwrap();
                                    println!("Successfully authenticated user {}", id);
                                    HttpResponse::Found()
                                        .header(actix_web::http::header::LOCATION, "/")
                                        .json(body)
                                } else {
                                    unreachable!()
                                }
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
    match session.get("userId") {
        Ok(Some(id)) => match User::by_id(id) {
            Ok(user) => HttpResponse::Ok().json(OkMessage {
                ok: true,
                message: user.map(UserResponse::from),
            }),
            Err(e) => HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some(e.to_string()),
            }),
        },
        _ => HttpResponse::Unauthorized().json(OkMessage {
            ok: false,
            message: Some("No user logged in"),
        }),
    }
}
