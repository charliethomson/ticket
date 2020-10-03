use {
    crate::{
        db::{
            models::Customer,
            schema::{CustomerOptions, Update},
        },
        routes::OkMessage,
    },
    actix_web::{get, post, put, web::Json, HttpResponse},
    serde::{Deserialize, Serialize},
};
#[derive(Serialize, Deserialize)]
pub struct CustomerNew {
    name: String,
    phone_number: String,
    email_address: String,
    store_id: i64,
}

#[post("/api/customers")]
pub async fn customers_post(Json(body): Json<CustomerNew>) -> HttpResponse {
    match Customer::insert(&Customer {
        id: 0,
        name: body.name,
        phone_number: body.phone_number,
        email: body.email_address,
        store_id: body.store_id,
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
#[get("/api/customers")]
pub async fn customers_get(Json(filter): Json<CustomerOptions>) -> HttpResponse {
    match Customer::find(filter) {
        Ok(customer) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: customer,
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

// TODO
#[put("/api/customers")]
pub async fn customers_put(Json(body): Json<CustomerOptions>) -> HttpResponse {
    let id = match body.id {
        Some(id) => id,
        None => {
            return HttpResponse::BadRequest().json(OkMessage {
                ok: false,
                message: Some("Missing required field `id`"),
            })
        }
    };

    match Customer::by_id(id) {
        Ok(Some(mut customer)) => match customer.update(body) {
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
            message: Some(format!("No customer found for id {}", id)),
        }),
        Err(e) => HttpResponse::Ok().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}
