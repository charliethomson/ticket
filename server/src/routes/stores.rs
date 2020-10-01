use {
    crate::{
        db::{models::Store, schema::StoreOptions},
        routes::OkMessage,
    },
    actix_web::{get, post, put, web::Json, HttpResponse},
    serde::{Deserialize, Serialize},
};
#[derive(Serialize, Deserialize)]
pub struct StoreNew {
    name: String,
    contact_name: String,
    phone_number: String,
    email: String,
    address: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Serialize, Deserialize)]
pub struct StoreUpdate {
    id: i64,
    name: Option<String>,
    contact_name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
}

#[post("/api/stores")]
pub async fn stores_post(Json(body): Json<StoreNew>) -> HttpResponse {
    match Store::insert(&Store {
        id: 0,
        name: body.name,
        contact_name: body.contact_name,
        phone_number: body.phone_number,
        email: body.email,
        address: body.address,
        city: body.city,
        state: body.state,
        zip: body.zip,
    }) {
        Ok(id) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: Some(id),
        }),
        Err(e) => HttpResponse::Ok().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

#[get("/api/stores")]
pub async fn stores_get(Json(filter): Json<StoreOptions>) -> HttpResponse {
    eprintln!("{}", filter.into_filter());
    match Store::find(filter) {
        Ok(option) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: option,
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

#[put("/api/stores")]
pub async fn stores_put(Json(_body): Json<StoreUpdate>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
