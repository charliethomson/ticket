use {
    crate::{
        db::{models::Customer, schema::CustomerOptions},
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

#[derive(Serialize, Deserialize)]
pub struct CustomerUpdate {
    id: i64,
    name: Option<String>,
    phone_number: Option<String>,
    email_address: Option<String>,
    store_id: Option<i64>,
}

// TODO
#[post("/api/customers")]
pub async fn customers_post(Json(_body): Json<CustomerNew>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO
#[get("/api/customers")]
pub async fn customers_get(Json(_filter): Json<CustomerOptions>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO
#[put("/api/customers")]
pub async fn customers_put(Json(_body): Json<CustomerUpdate>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
