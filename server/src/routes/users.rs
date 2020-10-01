use crate::{
    db::{models::User, schema::UserFind},
    routes::OkMessage,
};
use actix_web::{get, post, put, web, HttpResponse};

#[post("/api/users")]
async fn users_post(_body: Option<web::Json<UserFind>>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/api/users")]
async fn users_get(body: Option<web::Json<UserFind>>) -> HttpResponse {
    let filter = body.map(|json| json.into_inner()).unwrap_or_default();
    match User::find(filter) {
        Ok(user) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: user,
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

#[put("/api/users")]
async fn users_put(_body: web::Json<UserFind>) -> HttpResponse {
    let options = _body.into_inner();
    let mut user = User::by_id(options.id.unwrap()).unwrap().unwrap();
    match user.update(options) {
        Ok(successful) => HttpResponse::Ok().json(OkMessage::<()> {
            ok: successful,
            message: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}
