use {
    crate::{
        build_query, check_logged_in,
        db::{schema::users::dsl::*, User, UserFilter, UserNew, UserResponse, UserUpdate},
        not_ok, ok,
        routes::OkMessage,
    },
    actix_identity::Identity,
    actix_web::{
        get, post, put,
        web::{Json, Query},
        HttpResponse,
    },
    diesel::prelude::*,
};

#[post("/api/users")]
pub async fn users_post(identity: Identity, Json(body): Json<UserNew>) -> HttpResponse {
    check_logged_in!(identity, {
        match diesel::insert_into(users)
            .values(body)
            .execute(&crate::db::establish_connection())
        {
            Ok(inserted) => HttpResponse::Ok().json(OkMessage {
                ok: true,
                message: Some(inserted),
            }),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[get("/api/users")]
pub async fn users_get(identity: Identity, Query(filter): Query<UserFilter>) -> HttpResponse {
    check_logged_in!(identity, {
        use crate::db::schema::users as users_table;
        let query = build_query!(users_table, filter => {
            id,
            first_name,
            last_name,
            email_address
        });

        match query.get_results::<User>(&crate::db::establish_connection()) {
            Ok(results) => HttpResponse::Ok().json(ok!(results
                .into_iter()
                .map(UserResponse::from)
                .collect::<Vec<UserResponse>>())),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[put("/api/users")]
pub async fn users_put(identity: Identity, Json(body): Json<UserUpdate>) -> HttpResponse {
    check_logged_in!(identity, {
        match diesel::update(users)
            .filter(id.eq(body.id))
            .set(body)
            .execute(&crate::db::establish_connection())
        {
            Ok(_updated_row) => HttpResponse::Ok().json(ok!(_updated_row)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
