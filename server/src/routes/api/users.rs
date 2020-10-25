use {
    crate::{
        check_logged_in,
        db::{models::User, schema::UserOptions, Update},
        not_ok, ok,
        routes::OkMessage,
        validate_ok,
    },
    actix_identity::Identity,
    actix_web::{get, post, put, web, HttpResponse},
    lazy_static::lazy_static,
    regex::Regex,
    serde::{Deserialize, Serialize},
    webforms::validate::*,
};

#[derive(Serialize, Deserialize, ValidateForm)]
pub struct UserNew {
    pub google_id: i128,
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
}

#[post("/api/users")]
pub async fn users_post(identity: Identity, web::Json(body): web::Json<UserNew>) -> HttpResponse {
    check_logged_in!(identity, {
        validate_ok!(body, {
            match User::insert(body) {
                Ok(id) => HttpResponse::Ok().json(OkMessage {
                    ok: true,
                    message: Some(id),
                }),
                Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
            }
        })
    })
}

// TODO REMOVE
#[post("/api/users/internal")]
pub async fn users_post_internal(
    // TODO REMOVE
    web::Json(body): web::Json<UserNew>,
    // TODO REMOVE
) -> HttpResponse {
    validate_ok!(body, {
        // TODO REMOVE
        match User::insert(body) {
            // TODO REMOVE
            Ok(id) => HttpResponse::Ok().json(ok!(id)),
            // TODO REMOVE
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
        // TODO REMOVE
    })
}

#[get("/api/users")]
pub async fn users_get(identity: Identity, body: Option<web::Query<UserOptions>>) -> HttpResponse {
    check_logged_in!(identity, {
        let filter = body.map(|json| json.into_inner()).unwrap_or_default();
        match User::find(filter) {
            Ok(user) => HttpResponse::Ok().json(ok!(user)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[put("/api/users")]
pub async fn users_put(identity: Identity, body: web::Json<UserOptions>) -> HttpResponse {
    check_logged_in!(identity, {
        let options = body.into_inner();
        let user_id = match options.id {
            Some(id) => id,
            None => return HttpResponse::BadRequest().json(not_ok!("Missing required field `id`")),
        };
        let mut user = match User::by_id(user_id) {
            Ok(Some(user)) => user,
            Ok(None) => {
                return HttpResponse::NotFound()
                    .json(not_ok!(format!("User with id {} not found", user_id)))
            }
            Err(e) => return HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        };
        match user.update(options) {
            Ok(()) => HttpResponse::Ok().json(ok!()),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
