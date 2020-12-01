use {
    crate::{
        check_logged_in,
        db::{
            schema::workorders::dsl::*, IntoQuery, Workorder, WorkorderFilter, WorkorderNew,
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
            let conn = crate::db::establish_connection();
            match diesel::insert_into(workorders).values(body).execute(&conn) {
                Ok(_) => HttpResponse::Ok().json(ok!(crate::db::last_inserted(&conn))),
                Err(e) => HttpResponse::InternalServerError().json(OkMessage {
                    ok: false,
                    message: Some(e.to_string()),
                }),
            }
        })
    })
}

#[get("/api/workorders")]
pub async fn workorders_get(identity: Identity, req: actix_web::HttpRequest) -> HttpResponse {
    check_logged_in!(identity, {
        let filter: WorkorderFilter = serde_qs::from_str(req.query_string()).unwrap();
        let query = filter.into_query();

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
