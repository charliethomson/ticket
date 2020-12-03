use {
    crate::{
        check_logged_in,
        db::{
            establish_connection, last_inserted, schema::stores::dsl::*, Store, StoreFilter,
            StoreNew, StoreUpdate,
        },
        not_ok, ok,
        routes::{Limit, OkMessage},
        validate_ok,
    },
    actix_identity::Identity,
    actix_web::{
        get, post, put,
        web::{Json, Query},
        HttpResponse,
    },
    diesel::prelude::*,
    into_query::IntoQuery,
    webforms::validate::*,
};

#[post("/api/stores")]
pub async fn stores_post(identity: Identity, Json(body): Json<StoreNew>) -> HttpResponse {
    check_logged_in!(identity, {
        validate_ok!(body, {
            let conn = establish_connection();
            match diesel::insert_into(stores).values(body).execute(&conn) {
                Ok(_) => HttpResponse::Ok().json(ok!(last_inserted(&conn))),
                Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
            }
        })
    })
}

#[get("/api/stores")]
pub async fn stores_get(
    identity: Identity,
    Query(filter): Query<StoreFilter>,
    Query(limit): Query<Limit>,
) -> HttpResponse {
    check_logged_in!(identity, {
        let query = filter.into_query();

        match query
            .limit(limit.into())
            .get_results::<Store>(&establish_connection())
        {
            Ok(results) => HttpResponse::Ok().json(ok!(results)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[put("/api/stores")]
pub async fn stores_put(identity: Identity, Json(body): Json<StoreUpdate>) -> HttpResponse {
    check_logged_in!(identity, {
        match diesel::update(stores)
            .set(body)
            .execute(&establish_connection())
        {
            Ok(_updated_row) => HttpResponse::Ok().json(ok!(_updated_row)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
