use crate::routes::{AppState, OkMessage, UserNew};
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

    let token = data
        .oauth_client
        .exchange_code(code)
        // TODO: Verifier p/ty
        // .set_pkce_verifier(PkceCodeVerifier::new(data.verifier.secret().clone()))
        .request(http_client);

    let client = Client::builder()
        .connector(
            Connector::new()
                .ssl(SslConnector::builder(SslMethod::tls()).unwrap().build())
                .finish(),
        )
        .finish();
    match client
        .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .bearer_auth(token.unwrap().access_token().secret())
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
                        Ok(mut res) => {
                            // println!("{:#?}", res.body().await.unwrap().slice(..));
                            match res.json::<OkMessage<i64>>().await {
                                Ok(body) => HttpResponseBuilder::new(res.status()).json(body),
                                Err(e) => HttpResponseBuilder::new(res.status()).json(OkMessage {
                                    ok: false,
                                    message: Some(e.to_string()),
                                }),
                            }
                        }
                        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
                            ok: false,
                            message: Some(e.to_string()),
                        }),
                    }
                }
                _ => {
                    // TODO
                    HttpResponse::Unauthorized().body(format!("Unable to login with that email :("))
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

#[get("/api/auth/me")]
pub async fn auth_me(session: Session) -> HttpResponse {
    let user_id: Option<i64> = session.get("loggedInUser").unwrap();
    HttpResponse::Ok().json(user_id)
}
