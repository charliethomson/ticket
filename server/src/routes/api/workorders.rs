use {
    crate::{db::*, routes::OkMessage, validate_ok},
    actix_web::{
        get, post, put,
        web::{Json, Query},
        HttpResponse,
    },
    chrono::Utc,
    serde::{Deserialize, Serialize},
    webforms::validate::*,
};

#[derive(Serialize, Deserialize, ValidateForm)]
pub struct WorkorderNew {
    pub origin: i64,
    pub travel_status: i64,
    pub quoted_time: Option<i64>,
    pub status: i64,
    pub location: Option<String>,
    pub customer: i64, // Customer ID
    pub device: i64,   // Device ID
    #[validate(max_length = 144)]
    pub brief: String,
    pub initial_note: InitialNote,
}

#[derive(Serialize, Deserialize)]
pub struct InitialNote {
    pub user: i64,
    pub contents: String,
    pub next_update: Option<i64>,
}

// API call to create and handle making a new workorder
#[post("/api/workorders")]
pub async fn workorders_post(Json(body): Json<WorkorderNew>) -> HttpResponse {
    validate_ok!(body, {
        let note = Note {
            user: body.initial_note.user,
            created: Utc::now().timestamp(),
            next_update: body.initial_note.next_update,
            contents: body.initial_note.contents.clone(),
        };

        let mut conn = match crate::db::get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                return HttpResponse::InternalServerError().json(OkMessage {
                    ok: false,
                    message: Some(e.to_string()),
                })
            }
        };

        // this is a solid enough approximation of the workorder id, leaving a FIXME JIC
        // FIXME:
        use mysql::prelude::Queryable;
        let wo_id = match conn
            .query_first::<Option<i64>, &'static str>("select max(id) as max_id from workorders")
        {
            Ok(Some(Some(prev_max))) => prev_max + 1,
            Ok(_) => 1,
            Err(e) => {
                return HttpResponse::InternalServerError().json(OkMessage {
                    ok: false,
                    message: Some(e.to_string()),
                })
            }
        };

        if let Err(e) = note.insert(wo_id) {
            return HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some(e.to_string()),
            });
        }
        let wo = Workorder {
            workorder_id: 0,
            active: true,
            origin: body.origin,
            travel_status: body.travel_status,
            created: Utc::now().timestamp(),
            quoted_time: body.quoted_time,
            status: body.status,
            location: body.location.clone(),
            customer: body.customer,
            device: body.device,
            brief: body.brief.clone(),
        };

        match wo.insert() {
            Ok(id) => HttpResponse::Ok().json(OkMessage {
                ok: true,
                message: id,
            }),
            Err(e) => HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some(e.to_string()),
            }),
        }
    })
}

#[get("/api/workorders")]
pub async fn workorders_get(body: Option<Query<WorkorderOptions>>) -> HttpResponse {
    let filter = body.map(|body| body.into_inner()).unwrap_or_default();
    let response = Workorder::find(filter);

    match response {
        Ok(res) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: res,
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

#[put("/api/workorders")]
pub async fn workorders_put(Json(body): Json<WorkorderOptions>) -> HttpResponse {
    let id = match body.id {
        Some(id) => id,
        None => {
            return HttpResponse::BadRequest().json(OkMessage {
                ok: false,
                message: Some("Required option `id` not found"),
            })
        }
    };
    match Workorder::by_id(id) {
        Ok(Some(Ok(mut workorder))) => match workorder.update(body) {
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
            message: Some(format!("No workorder found for id {}", id)),
        }),
        Ok(Some(Err(e))) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e),
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}
