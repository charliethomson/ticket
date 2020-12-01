use {
    crate::{
        check_logged_in,
        db::{
            establish_connection, last_inserted, schema::users::dsl::*, IntoQuery, User,
            UserFilter, UserNew, UserResponse, UserUpdate,
        },
        not_ok, ok,
        routes::{Limit, OkMessage},
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
        let conn = establish_connection();
        match diesel::insert_into(users).values(body).execute(&conn) {
            Ok(_) => HttpResponse::Ok().json(ok!(last_inserted(&conn))),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[get("/api/users")]
pub async fn users_get(
    identity: Identity,
    Query(filter): Query<UserFilter>,
    Query(limit): Query<Limit>,
) -> HttpResponse {
    check_logged_in!(identity, {
        let query = filter.into_query();
        match query
            .limit(limit.into())
            .get_results::<User>(&establish_connection())
        {
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
