use {
    crate::{db::*, routes::OkMessage},
    actix_web::{get, post, put, web::Json, HttpResponse},
    chrono::{DateTime, NaiveDateTime, Utc},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
pub struct WorkorderNew {
    pub origin: i64,
    pub travel_status: String,
    pub quoted_time: Option<i64>,
    pub status: String,
    pub customer: i64, // Customer ID
    pub device: i64,   // Device ID
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
pub async fn workorders_post(body: Json<WorkorderNew>) -> HttpResponse {
    let note = Note {
        user: body.initial_note.user,
        created: Utc::now(),
        next_update: body
            .initial_note
            .next_update
            .map(|stamp| DateTime::from_utc(NaiveDateTime::from_timestamp(stamp, 0), Utc)),
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
        workorder_id: None,
        origin: body.origin,
        travel_status: body.travel_status.clone(),
        created: Utc::now(),
        quoted_time: body
            .quoted_time
            .map(|stamp| DateTime::from_utc(NaiveDateTime::from_timestamp(stamp, 0), Utc)),
        status: body.status.clone(),
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
}

#[get("/api/workorders")]
pub async fn workorders_get(body: Option<Json<WorkorderOptions>>) -> HttpResponse {
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
pub async fn workorders_put(_body: Option<Json<WorkorderOptions>>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
