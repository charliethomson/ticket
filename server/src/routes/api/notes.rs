use {
    crate::{
        check_logged_in,
        db::{last_inserted, IntoQuery, Note, NoteFilter, NoteResponse, NotesNew},
        not_ok, ok,
        routes::OkMessage,
    },
    actix_identity::Identity,
    actix_web::{
        get, post,
        web::{Json, Query},
        HttpResponse,
    },
    diesel::prelude::*,
};

#[post("/api/notes")]
pub async fn notes_post(identity: Identity, Json(body): Json<NotesNew>) -> HttpResponse {
    check_logged_in!(identity, {
        use crate::db::schema::notes;
        let conn = crate::db::establish_connection();
        match diesel::insert_into(notes::dsl::notes)
            .values(body)
            .execute(&conn)
        {
            Ok(_) => HttpResponse::Ok().json(ok!(last_inserted(&conn))),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}

#[get("/api/notes")]
pub async fn notes_get(identity: Identity, Query(body): Query<NoteFilter>) -> HttpResponse {
    check_logged_in!(identity, {
        // TODO: Make this prettier :)
        let query = body.into_query();

        match query.get_results::<Note>(&crate::db::establish_connection()) {
            Ok(results) => HttpResponse::Ok().json(ok!(results
                .into_iter()
                .map(NoteResponse::from)
                .collect::<Vec<NoteResponse>>())),
            Err(e) => HttpResponse::InternalServerError().json(not_ok!(e.to_string())),
        }
    })
}
