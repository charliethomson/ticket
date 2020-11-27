use {
    crate::{
        build_query, check_logged_in,
        db::{schema::stores::dsl::*, Store, StoreFilter, StoreNew, StoreUpdate},
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
    diesel::prelude::*,
    webforms::validate::*,
};

#[post("/api/stores")]
pub async fn stores_post(identity: Identity, Json(body): Json<StoreNew>) -> HttpResponse {
    check_logged_in!(identity, {
        validate_ok!(body, {
            match diesel::insert_into(stores)
                .values(body)
                .execute(&crate::db::establish_connection())
            {
                Ok(inserted) => HttpResponse::Ok().json(ok!(inserted)),
                Err(e) => HttpResponse::InternalServerError().json(OkMessage {
                    ok: false,
                    message: Some(e.to_string()),
                }),
            }
        })
    })
}

#[get("/api/stores")]
pub async fn stores_get(identity: Identity, filter: Query<StoreFilter>) -> HttpResponse {
    check_logged_in!(identity, {
        use crate::db::schema::stores as stores_table;
        let query = build_query!(stores_table, filter => {
            id,
            contact_name,
            phone_number,
            email_address,
            address,
            city,
            state,
            zip
        });

        match query.get_results::<Store>(&crate::db::establish_connection()) {
            Ok(results) => HttpResponse::Ok().json(ok!(results)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[put("/api/stores")]
pub async fn stores_put(identity: Identity, Json(body): Json<StoreUpdate>) -> HttpResponse {
    check_logged_in!(identity, {
        match diesel::update(stores)
            .set(body)
            .execute(&crate::db::establish_connection())
        {
            Ok(_updated_row) => HttpResponse::Ok().json(ok!(_updated_row)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
