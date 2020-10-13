use crate::{
    db::{models::User, schema::UserOptions, Update},
    routes::OkMessage,
};
use actix_session::Session;
use actix_web::{get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserNew {
    pub google_id: i128,
    pub name: String,
    pub phone_number: String,
    pub email: String,
}

#[post("/api/users")]
pub async fn users_post(_session: Session, body: web::Json<UserNew>) -> HttpResponse {
    // TODO: Check auth
    /*match session.get::<bool>("authenticated") {
    Ok(Some(true)) =>*/
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
    //     Ok(_) => HttpResponse::Unauthorized().json(OkMessage {
    //         ok: false,
    //         message: Some(format!("Authorization check failed")),
    //     }),
    //     Err(e) => HttpResponse::InternalServerError().json(OkMessage {
    //         ok: false,
    //         message: Some(e.to_string()),
    //     }),
    // }
}

#[get("/api/users")]
pub async fn users_get(body: Option<web::Query<UserOptions>>) -> HttpResponse {
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
            return HttpResponse::BadRequest().json(OkMessage {
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
        Ok(()) => HttpResponse::Ok().json(OkMessage::<()> {
            ok: true,
            message: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}
