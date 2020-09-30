use {
    crate::{db::*, routes::OkMessage},
    actix_web::{get, post, web, HttpResponse, Responder},
    chrono::{serde::ts_seconds, DateTime, Utc},
    futures::stream::StreamExt,
    serde::{Deserialize, Serialize},
    std::convert::TryFrom,
};

/*
{
    "origin": 1,
    "travel_status": "Delivered",
    "quoted_time": null,
    "status": "Awaiting repair",
    "customer": 1,
    "device": 1,
    "brief": "broken",
    "initial_note": {
        "user": 1,
        "contents": "it no workie",
        "next_update": null
    }
}
*/

#[derive(Serialize, Deserialize)]
pub struct WorkorderNew {
    pub origin: i64,
    pub travel_status: String,
    pub quoted_time: Option<DateTime<Utc>>,
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
    pub next_update: Option<DateTime<Utc>>,
}

// API call to create and handle making a new workorder
#[post("/api/workorders/new")]
async fn workorder_new(body: web::Json<WorkorderNew>) -> impl Responder {
    let note = Note {
        user: body.initial_note.user,
        created: Utc::now(),
        next_update: body.initial_note.next_update,
        contents: body.initial_note.contents.clone(),
    };

    // TODO;
    let wo_id: i64 = crate::db::get_connection()
        .unwrap()
        .query_first::<Option<i64>, &'static str>("select max(id) as max_id from workorders")
        .unwrap()
        .unwrap()
        .unwrap_or(0)
        + 1;

    // TODO;
    let res = match note.insert(wo_id) {
        Err(e) => {
            return HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some(e.to_string()),
            })
        }
        Ok(result) => result,
    };

    eprintln!("{:?}", res);

    let wo = Workorder {
        workorder_id: None,
        origin: body.origin,
        travel_status: body.travel_status.clone(),
        created: Utc::now(),
        quoted_time: body.quoted_time,
        status: body.status.clone(),
        customer: body.customer,
        device: body.device,
        brief: body.brief.clone(),
    };

    let result = wo.insert();
    eprintln!("{:?}", result);

    match result {
        Ok(id) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: Some(id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

#[get("/api/workorders/find")]
async fn workorders_find(body: web::Json<WorkorderFind>) -> impl Responder {
    let response = Workorder::find(body.into_inner());

    match response {
        Ok(res) => HttpResponse::Ok().json(OkMessage {
            ok: res.is_some(),
            message: res,
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

// API call to get all workorders in the table without any filtering
#[get("/api/workorders/all")]
async fn workorders_all() -> impl Responder {
    let response = Workorder::find(WorkorderFind::default());
    match response {
        Ok(res) => HttpResponse::Ok().json(OkMessage {
            ok: res.is_some(),
            message: res,
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

// API call to get a workorder by its ID
#[get("/api/workorders/{id}")]
async fn workorder_by_id(web::Path(id): web::Path<i64>) -> impl Responder {
    let wo = crate::db::models::Workorder::by_id(id);
    match wo {
        Ok(wo) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: wo,
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}
