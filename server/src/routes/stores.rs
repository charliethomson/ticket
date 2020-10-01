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
pub async fn stores_put(Json(body): Json<StoreOptions>) -> HttpResponse {
    let id = match body.id {
        Some(id) => id,
        None => {
            return HttpResponse::PartialContent().json(OkMessage {
                ok: false,
                message: Some("Required option `id` not found"),
            })
        }
    };
    match Store::by_id(id) {
        Ok(Some(mut store)) => match store.update(body) {
            Ok(_) => HttpResponse::Ok().json(OkMessage::<()> {
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
            message: Some(format!("No store found for id {}", id)),
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}
