use {
    crate::{
        check_logged_in,
        db::{models::Customer, schema::CustomerOptions, Insert, Update},
        not_ok, ok,
        routes::OkMessage,
        validate_ok,
    },
    actix_identity::Identity,
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
pub async fn customers_post(identity: Identity, Json(body): Json<CustomerNew>) -> HttpResponse {
    check_logged_in!(identity, {
        validate_ok!(body, {
            match Customer::insert(&Customer {
                id: 0,
                first_name: body.first_name,
                last_name: body.last_name,
                phone_number: body.phone_number,
                email: body.email_address,
            }) {
                Ok(id) => HttpResponse::Ok().json(ok!(id)),
                Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
            }
        })
    })
}

// TODO
#[get("/api/customers")]
pub async fn customers_get(identity: Identity, filter: Query<CustomerOptions>) -> HttpResponse {
    check_logged_in!(identity, {
        match Customer::find(filter.into_inner()) {
            Ok(customer) => HttpResponse::Ok().json(ok!(customer)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

// TODO
#[put("/api/customers")]
pub async fn customers_put(identity: Identity, Json(body): Json<CustomerOptions>) -> HttpResponse {
    check_logged_in!(identity, {
        let id = match body.id {
            Some(id) => id,
            None => return HttpResponse::BadRequest().json(not_ok!("Missing required field `id`")),
        };

        match Customer::by_id(id) {
            Ok(Some(mut customer)) => match customer.update(body) {
                Ok(()) => HttpResponse::Ok().json(ok!(())),
                Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
            },
            Ok(None) => {
                HttpResponse::NotFound().json(ok!(format!("No customer found for id {}", id)))
            }
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
