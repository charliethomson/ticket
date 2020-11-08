use {
    crate::{check_logged_in, db::*, not_ok, ok, routes::OkMessage, validate_ok},
    actix_identity::Identity,
    actix_web::{get, post, put, web::Json, HttpRequest, HttpResponse},
    chrono::Utc,
    serde::{Deserialize, Serialize},
    webforms::validate::*,
};

#[derive(Serialize, Deserialize, ValidateForm)]
pub struct WorkorderNew {
    pub origin: i64,
    pub travel_status: Option<i64>,
    pub quoted_time: Option<i64>,
    pub status: Option<i64>,
    pub location: Option<String>,
    pub customer: i64, // Customer ID
    pub device: i64,   // Device ID
    #[validate(max_length = 144)]
    pub brief: String,
}

// API call to create and handle making a new workorder
#[post("/api/workorders")]
pub async fn workorders_post(identity: Identity, Json(body): Json<WorkorderNew>) -> HttpResponse {
    check_logged_in!(identity, {
        validate_ok!(body, {
            let note = Note {
                // TODO;
                user: identity.identity().unwrap_or("1".into()).parse().unwrap(),
                created: Utc::now().timestamp(),
                next_update: None,
                contents: body.brief.clone(),
            };

            let mut conn = match crate::db::get_connection() {
                Ok(conn) => conn,
                Err(e) => return HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
            };

            // this is a solid enough approximation of the workorder id, leaving a FIXME JIC
            // FIXME:
            // use mysql::prelude::Queryable;
            // let wo_id = match conn.query_first::<Option<i64>, &'static str>(
            //     "select max(id) as max_id from workorders",
            // ) {
            //     Ok(Some(Some(prev_max))) => prev_max + 1,
            //     Ok(_) => 1,
            //     Err(e) => return HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
            // };
            // println!("Note: {:#?}, wo_id: {}", note, wo_id);
            // if let Err(e) = note.insert(wo_id) {
            //     return HttpResponse::InternalServerError().json(not_ok!(e.to_string()));
            // }
            let wo = Workorder {
                id: 0,
                active: true,
                origin: body.origin,
                // TODO: Get default value from justin
                travel_status: body.travel_status.unwrap_or(0),
                created: Utc::now().timestamp(),
                quoted_time: body.quoted_time,
                // TODO: Get default value from justin
                status: body.status.unwrap_or(0),
                location: body.location.clone(),
                customer: body.customer,
                device: body.device,
                brief: body.brief.clone(),
            };

            match wo.insert() {
                Ok(id) => {
                    if let Some(id) = id {
                        if let Err(e) = note.insert(id) {
                            return HttpResponse::InternalServerError()
                                .json(not_ok!(e.to_string()));
                        } else {
                            HttpResponse::Ok().json(ok!(id))
                        }
                    } else {
                        unreachable!()
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
            }
        })
    })
}

#[get("/api/workorders")]
pub async fn workorders_get(identity: Identity, req: HttpRequest) -> HttpResponse {
    // TODO
    check_logged_in!(identity, {
        let filter = match serde_qs::from_str::<WorkorderOptions>(&req.query_string()) {
            Ok(filter) => filter,
            Err(e) => return HttpResponse::BadRequest().json(not_ok!(e.to_string())),
        };
        let response = Workorder::find(filter);

        match response {
            Ok(res) => HttpResponse::Ok().json(ok!(res)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[put("/api/workorders")]
pub async fn workorders_put(
    identity: Identity,
    Json(body): Json<WorkorderOptions>,
) -> HttpResponse {
    check_logged_in!(identity, {
        let id = match body.id {
            Some(id) => id,
            None => {
                return HttpResponse::BadRequest().json(not_ok!("Required option `id` not found"))
            }
        };
        match Workorder::by_id(id) {
            Ok(Some(Ok(mut workorder))) => match workorder.update(body) {
                Ok(_) => HttpResponse::Ok().json(ok!()),
                Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
            },
            Ok(None) => {
                HttpResponse::NotFound().json(not_ok!(format!("No workorder found for id {}", id)))
            }
            Ok(Some(Err(e))) => HttpResponse::InternalServerError().json(not_ok!(e)),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
