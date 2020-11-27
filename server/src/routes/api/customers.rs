use {
    crate::{
        build_query, check_logged_in,
        db::{schema::customers::dsl::*, Customer, CustomerFilter, CustomerNew, CustomerUpdate},
        not_ok, ok,
        routes::{api::Limit, OkMessage},
        validate_ok,
    },
    actix_identity::Identity,
    actix_web::{
        get, post, put,
        web::{Json, Query},
        HttpResponse,
    },
    diesel::prelude::*,
    webforms::validate::*,
};

#[post("/api/customers")]
pub async fn customers_post(identity: Identity, Json(body): Json<CustomerNew>) -> HttpResponse {
    check_logged_in!(identity, {
        validate_ok!(body, {
            match diesel::insert_into(customers)
                .values(&body)
                .execute(&crate::db::establish_connection())
            {
                Ok(inserted) => HttpResponse::Ok().json(ok!(inserted)),
                Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
            }
        })
    })
}

#[get("/api/customers")]
pub async fn customers_get(
    identity: Identity,
    Query(filter): Query<CustomerFilter>,
    Query(limit): Query<Limit>,
) -> HttpResponse {
    check_logged_in!(identity, {
        use crate::db::schema::customers as customers_table;
        let query = build_query!(customers_table, filter => {
            id,
            first_name,
            last_name,
            phone_number,
            email_address
        });

        match query.get_results::<Customer>(&crate::db::establish_connection()) {
            Ok(results) => HttpResponse::Ok().json(ok!(results)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[put("/api/customers")]
pub async fn customers_put(identity: Identity, Json(body): Json<CustomerUpdate>) -> HttpResponse {
    check_logged_in!(identity, {
        match diesel::update(customers)
            .filter(id.eq(body.id))
            .set(body)
            .execute(&crate::db::establish_connection())
        {
            Ok(_updated_row) => HttpResponse::Ok().json(ok!(_updated_row)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

// TODO
// #[get("/api/customers/query")]
// pub async fn customers_query(identity: Identity, query: Query<String>) -> HttpResponse {
//     check_logged_in!(identity, {
//         let filter = CustomerOptions {
//             first_name: Some(query.to_string()),
//             last_name: Some(query.to_string()),
//             phone_number: Some(query.to_string()),
//             email: Some(query.to_string()),
//             ..CustomerOptions::default()
//         };
//         match Customer::find(filter) {
//             Ok(customer) => HttpResponse::Ok().json(ok!(customer)),
//             Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
//         }
//     })
// }
