use {
    crate::{
        db::{models::Customer, schema::CustomerOptions, Insert, Update},
        routes::OkMessage,
        validate_ok,
    },
    actix_web::{
        get, post, put,
        web::{Json, Query},
        HttpResponse,
    },
    lazy_static::lazy_static,
    regex::Regex,
    serde::{Deserialize, Serialize},
    webforms::validate::*,
};

#[derive(Serialize, Deserialize, ValidateForm)]
pub struct CustomerNew {
    first_name: String,
    last_name: String,
    #[validate(phone)]
    phone_number: String,
    #[validate(email)]
    email_address: String,
}

#[post("/api/customers")]
pub async fn customers_post(Json(body): Json<CustomerNew>) -> HttpResponse {
    validate_ok!(body, {
        match Customer::insert(&Customer {
            id: 0,
            first_name: body.first_name,
            last_name: body.last_name,
            phone_number: body.phone_number,
            email: body.email_address,
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
    })
}

// TODO
#[get("/api/customers")]
pub async fn customers_get(filter: Query<CustomerOptions>) -> HttpResponse {
    match Customer::find(filter.into_inner()) {
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
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}
