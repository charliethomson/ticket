use {
    crate::{
        db::{
            models::{Note, User},
            schema::NotesOptions,
        },
        routes::OkMessage,
    },
    actix_web::{get, post, web::Json, HttpResponse},
    chrono::{DateTime, NaiveDateTime, Utc},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
pub struct NotesNew {
    workorder_id: i64,
    user: User,
    contents: String,
    next_update: Option<i64>,
}

#[post("/api/notes")]
pub fn notes_post(Json(body): Json<NotesNew>) -> HttpResponse {
    let note = Note {
        user: body.user.id,
        created: Utc::now(),
        next_update: body
            .next_update
            .map(|stamp| DateTime::from_utc(NaiveDateTime::from_timestamp(stamp, 0), Utc)),
        contents: body.contents,
    };

    match note.insert(body.workorder_id) {
        Ok(id) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: Some(id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    };
    HttpResponse::Ok().finish()
}

#[get("/api/notes")]
pub fn notes_get(_body: Option<Json<NotesOptions>>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
