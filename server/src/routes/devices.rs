use {
    crate::{
        db::{models::Device, schema::DeviceOptions},
        routes::OkMessage,
    },
    actix_web::{get, post, put, web::Json, HttpResponse},
    serde::{Deserialize, Serialize},
};
#[derive(Serialize, Deserialize)]
pub struct DeviceNew {}

#[derive(Serialize, Deserialize)]
pub struct DeviceUpdate {}

// TODO
#[post("/api/devices")]
pub async fn devices_post(Json(_body): Json<DeviceNew>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO
#[get("/api/devices")]
pub async fn devices_get(Json(_filter): Json<DeviceOptions>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO
#[put("/api/devices")]
pub async fn devices_put(Json(_body): Json<DeviceUpdate>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
