use {
    crate::{
        check_logged_in,
        db::{Device, DeviceOptions, Insert, Update},
        not_ok, ok,
        routes::OkMessage,
    },
    actix_identity::Identity,
    actix_web::{
        get, post, put,
        web::{Json, Query},
        HttpResponse,
    },
    serde::{Deserialize, Serialize},
};
#[derive(Serialize, Deserialize)]
pub struct DeviceNew {
    pub serial: String,
    pub name: String,
    pub customer_id: i64, // Customer ID
    pub password: String,
}

#[post("/api/devices")]
pub async fn devices_post(identity: Identity, Json(body): Json<DeviceNew>) -> HttpResponse {
    check_logged_in!(identity, {
        match Device::insert(&Device {
            id: 0,
            serial: body.serial.clone(),
            name: body.name.clone(),
            customer_id: body.customer_id,
            password: body.password,
        }) {
            Ok(id) => HttpResponse::Ok().json(ok!(id)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[get("/api/devices")]
pub async fn devices_get(identity: Identity, filter: Query<DeviceOptions>) -> HttpResponse {
    check_logged_in!(identity, {
        match Device::find(filter.into_inner()) {
            Ok(devices) => HttpResponse::Ok().json(ok!(devices)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[put("/api/devices")]
pub async fn devices_put(identity: Identity, Json(body): Json<DeviceOptions>) -> HttpResponse {
    check_logged_in!(identity, {
        let id = match body.id {
            Some(id) => id,
            None => return HttpResponse::BadRequest().json(not_ok!("Missing required field `id`")),
        };

        match Device::by_id(id) {
            Ok(Some(mut device)) => match device.update(body) {
                Ok(()) => HttpResponse::Ok().json(ok!(())),
                Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
            },
            Ok(None) => {
                HttpResponse::NotFound().json(not_ok!(format!("No device found for id {}", id)))
            }
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
