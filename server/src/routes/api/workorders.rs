use {
    crate::{
        build_query, check_logged_in,
        db::{
            schema::workorders::dsl::*, Workorder, WorkorderFilter, WorkorderNew,
            WorkorderResponse, WorkorderUpdate,
        },
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

// API call to create and handle making a new workorder
#[post("/api/workorders")]
pub async fn workorders_post(identity: Identity, Json(body): Json<WorkorderNew>) -> HttpResponse {
    check_logged_in!(identity, {
        validate_ok!(body, {
            match diesel::insert_into(workorders)
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

#[get("/api/workorders")]
pub async fn workorders_get(
    identity: Identity,
    Query(filter): Query<WorkorderFilter>,
) -> HttpResponse {
    check_logged_in!(identity, {
        use crate::db::schema::workorders as workorders_table;
        let query = build_query!(workorders_table, filter => {
            id,
            active,
            origin,
            created,
            quoted,
            workorder_status,
            travel_status,
            location,
            customer,
            device,
            brief
        });

        match query.get_results::<Workorder>(&crate::db::establish_connection()) {
            Ok(results) => HttpResponse::Ok().json(ok!(results
                .into_iter()
                .map(WorkorderResponse::from)
                .collect::<Vec<WorkorderResponse>>())),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[put("/api/workorders")]
pub async fn workorders_put(identity: Identity, Json(body): Json<WorkorderUpdate>) -> HttpResponse {
    check_logged_in!(identity, {
        match diesel::update(workorders)
            .filter(id.eq(body.id))
            .set(body)
            .execute(&crate::db::establish_connection())
        {
            Ok(_updated_row) => HttpResponse::Ok().json(ok!(_updated_row)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
