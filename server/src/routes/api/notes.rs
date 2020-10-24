use {
    crate::{
        check_logged_in,
        db::{models::Note, schema::NotesOptions},
        routes::OkMessage,
    },
    actix_identity::Identity,
    actix_web::{
        get, post,
        web::{Json, Query},
        HttpResponse,
    },
    chrono::Utc,
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
pub struct NotesNew {
    workorder_id: i64,
    user_id: i64,
    contents: String,
    next_update: Option<i64>,
}

#[post("/api/notes")]
pub async fn notes_post(identity: Identity, Json(body): Json<NotesNew>) -> HttpResponse {
    check_logged_in!(identity, {
        let note = Note {
            user: body.user_id,
            created: Utc::now().timestamp(),
            next_update: body.next_update,
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
        }
    })
}

#[get("/api/notes")]
pub async fn notes_get(identity: Identity, body: Option<Query<NotesOptions>>) -> HttpResponse {
    check_logged_in!(identity, {
        let filter = body.map(|json| json.into_inner()).unwrap_or_default();
        match Note::find(filter) {
            Ok(notes) => HttpResponse::Ok().json(OkMessage {
                ok: true,
                message: notes,
            }),
            Err(e) => HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some(e.to_string()),
            }),
        }
    })
}
