use {
    crate::{
        db::{models::Device, schema::DeviceOptions},
        routes::OkMessage,
    },
    actix_web::{get, post, put, web::Json, HttpResponse},
    serde::{Deserialize, Serialize},
};
#[derive(Serialize, Deserialize)]
pub struct DeviceNew {
    pub serial: String,
    pub name: String,
    pub customer_id: i64, // Customer ID
    pub password: String,
}

// TODO
#[post("/api/devices")]
pub async fn devices_post(Json(body): Json<DeviceNew>) -> HttpResponse {
    match Device::insert(&Device {
        id: 0,
        serial: body.serial.clone(),
        name: body.name.clone(),
        customer_id: body.customer_id,
        password: body.password.clone(),
    }) {
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

// TODO
#[get("/api/devices")]
pub async fn devices_get(Json(filter): Json<DeviceOptions>) -> HttpResponse {
    match Device::find(filter) {
        Ok(devices) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: devices,
        }),
        Err(e) => HttpResponse::Ok().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

// TODO
#[put("/api/devices")]
pub async fn devices_put(Json(body): Json<DeviceOptions>) -> HttpResponse {
    let id = match body.id {
        Some(id) => id,
        None => {
            return HttpResponse::PartialContent().json(OkMessage {
                ok: false,
                message: Some("Missing required field `id`"),
            })
        }
    };

    match Device::by_id(id) {
        Ok(Some(mut device)) => match device.update(body) {
            Ok(()) => HttpResponse::Ok().json(OkMessage::<()> {
                ok: true,
                message: None,
            }),
            Err(e) => HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some(e.to_string()),
            }),
        },
        Ok(None) => HttpResponse::NotFound().json(OkMessage {
            ok: false,
            message: Some(format!("No device found for id {}", id)),
        }),
        Err(e) => HttpResponse::Ok().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}
