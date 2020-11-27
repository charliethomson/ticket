use {
    crate::{
        build_query, check_logged_in,
        db::{schema::devices::dsl::*, Device, DeviceFilter, DeviceNew, DeviceUpdate},
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
#[post("/api/devices")]
pub async fn devices_post(identity: Identity, Json(body): Json<DeviceNew>) -> HttpResponse {
    check_logged_in!(identity, {
        match diesel::insert_into(devices)
            .values(body)
            .execute(&crate::db::establish_connection())
        {
            Ok(inserted) => HttpResponse::Ok().json(ok!(inserted)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[get("/api/devices")]
pub async fn devices_get(identity: Identity, filter: Query<DeviceFilter>) -> HttpResponse {
    check_logged_in!(identity, {
        use crate::db::schema::devices as devices_table;
        let query = build_query!(devices_table, filter => {
           id,
           serial_no,
           device_name,
           customer,
           password
        });

        match query.get_results::<Device>(&crate::db::establish_connection()) {
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
            .execute(&crate::db::establish_connection())
        {
            Ok(_updated_row) => HttpResponse::Ok().json(ok!(_updated_row)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
