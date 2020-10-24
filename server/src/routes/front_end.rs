use actix_identity::Identity;
use actix_web::{get, HttpResponse};
use std::{fs::File, io::Read};

#[get("/")]
pub async fn front_end(identity: Identity) -> HttpResponse {
    eprintln!("\n\n\n\nuser_id: {:?}\n\n\n\n", identity.identity());
    let mut contents = String::new();
    let mut file = File::open("./static/public/index.html").unwrap();
    file.read_to_string(&mut contents).unwrap();

    HttpResponse::Ok().body(contents)
}
