use {
    crate::{
        check_logged_in,
        db::{
            establish_connection, last_inserted, schema::devices::dsl::*, Device, DeviceFilter,
            DeviceNew, DeviceUpdate,
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
    into_query::IntoQuery,
};
#[post("/api/devices")]
pub async fn devices_post(identity: Identity, Json(body): Json<DeviceNew>) -> HttpResponse {
    check_logged_in!(identity, {
        let conn = establish_connection();
        match diesel::insert_into(devices).values(body).execute(&conn) {
            Ok(_) => HttpResponse::Ok().json(ok!(last_inserted(&conn))),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[get("/api/devices")]
pub async fn devices_get(
    identity: Identity,
    Query(filter): Query<DeviceFilter>,
    Query(limit): Query<Limit>,
) -> HttpResponse {
    check_logged_in!(identity, {
        let query = filter.into_query();

        let conn = establish_connection();

        match query.limit(limit.into()).get_results::<Device>(&conn) {
            Ok(results) => HttpResponse::Ok().json(ok!(results)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[put("/api/devices")]
pub async fn devices_put(identity: Identity, Json(body): Json<DeviceUpdate>) -> HttpResponse {
    check_logged_in!(identity, {
        match diesel::update(devices)
            .set(body)
            .execute(&establish_connection())
        {
            Ok(_updated_row) => HttpResponse::Ok().json(ok!(_updated_row)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
