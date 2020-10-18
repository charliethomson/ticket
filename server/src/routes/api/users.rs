use {
    crate::{
        db::{models::User, schema::UserOptions, Update},
        routes::OkMessage,
        validate_ok,
    },
    actix_session::Session,
    actix_web::{get, post, put, web, HttpResponse},
    lazy_static::lazy_static,
    regex::Regex,
    serde::{Deserialize, Serialize},
    webforms::validate::*,
};

#[derive(Serialize, Deserialize, ValidateForm)]
pub struct UserNew {
    pub google_id: i128,
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[post("/api/users")]
pub async fn users_post(_session: Session, web::Json(body): web::Json<UserNew>) -> HttpResponse {
    // TODO: Check auth
    /*match session.get::<bool>("authenticated") {
    Ok(Some(true)) =>*/
    validate_ok!(body, {
        match User::insert(body) {
            Ok(id) => HttpResponse::Ok().json(OkMessage {
                ok: true,
                message: Some(id),
            }),
            Err(e) => HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some(e.to_string()),
            }),
        }
    })
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
