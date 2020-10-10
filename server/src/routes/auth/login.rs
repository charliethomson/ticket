use actix_session::Session;
use actix_web::{get, web, HttpResponse};
use oauth2::{
    curl::http_client, AccessToken, AuthorizationCode, CodeTokenRequest, CsrfToken,
    PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse,
};

#[get("/api/auth/login")]
pub async fn auth_login(data: web::Data<crate::routes::AppState>) -> HttpResponse {
    HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, data.auth_url.to_string())
        .finish()
}

#[get("/api/auth/response")]
pub async fn auth_response(
    session: Session,
    data: web::Data<crate::routes::AppState>,
    params: web::Query<super::AuthRequest>,
) -> HttpResponse {
    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());
    let scope = params.scope.clone();

    eprintln!("IN RESPONSE:\n{:#?}", data);

    let verifier = PkceCodeVerifier::new(data.verifier.secret().into());

    let token = &data
        .oauth_client
        .exchange_code(code)
        .set_pkce_verifier(verifier)
        .request(http_client);

    println!("{:?}", token);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://www.googleapis.com/auth/userinfo.profile")
        // .bearer_auth(token)
        .send();

    println!("{:?}", response);

    let user_id = 0;

    session.set("loggedInUser", user_id);
    println!("User id: {}", user_id);

    let html = r#""#;

    HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, "/")
        .finish()
}

#[get("/api/auth/me")]
pub async fn auth_me(session: Session) -> HttpResponse {
    let user_id: Option<i64> = session.get("loggedInUser").unwrap();
    HttpResponse::Ok().json(user_id)
}
