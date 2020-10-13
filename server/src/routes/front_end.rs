use actix_session::Session;
use actix_web::{get, HttpResponse};
use std::{fs::File, io::Read};

#[get("/")]
pub async fn front_end(session: Session) -> HttpResponse {
    eprintln!(
        "\n\n\n\nuser_id: {:?}\n\n\n\n",
        session.get::<i64>("userId")
    );
    let mut contents = String::new();
    let mut file = File::open("./static/index.html").unwrap();
    file.read_to_string(&mut contents).unwrap();

    HttpResponse::Ok().body(contents)
}
