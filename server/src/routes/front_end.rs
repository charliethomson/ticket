use actix_identity::Identity;
use actix_web::{get, HttpResponse};
use std::{fs::File, io::Read};

#[get("/")]
pub async fn front_end(identity: Identity) -> HttpResponse {
    // if identity.identity() == None {
    //     return HttpResponse::Ok()
    //         .header(actix_web::http::header::LOCATION, "/api/auth/login")
    //         .body("<p>This should redirect you, if it doesn't, click <a href=/api/auth/login>here</a>");
    // }
    let mut contents = String::new();
    let mut file = File::open("./static/public/index.html").unwrap();
    file.read_to_string(&mut contents).unwrap();

    HttpResponse::Ok().body(contents)
}
