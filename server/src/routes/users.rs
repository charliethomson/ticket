use crate::{
    db::{models::User, schema::UserOptions},
    routes::OkMessage,
};
use actix_web::{get, post, put, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserNew {
    pub name: String,
    pub phone_number: String,
}

#[post("/api/users")]
pub async fn users_post(body: web::Json<UserNew>) -> HttpResponse {
    match User::insert(body.into_inner()) {
        Ok(id) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: Some(id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

#[get("/api/users")]
pub async fn users_get(body: Option<web::Json<UserOptions>>) -> HttpResponse {
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
pub async fn users_put(body: web::Json<UserOptions>) -> HttpResponse {
    let options = body.into_inner();
    let user_id = match options.id {
        Some(id) => id,
        None => {
            return HttpResponse::PartialContent().json(OkMessage {
                ok: false,
                message: Some("Missing required field `id`"),
            })
        }
    };
    let mut user = match User::by_id(user_id) {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::NotFound().json(OkMessage {
                ok: false,
                message: Some(format!("User with id {} not found", user_id)),
            })
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some(e.to_string()),
            })
        }
    };
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
