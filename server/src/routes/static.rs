use actix_web::{get, HttpResponse};
use std::io::Read;

#[get("/")]
pub async fn index() -> HttpResponse {
    let mut content = String::new();
    let mut file_handle = std::fs::File::open("./static/index.html").unwrap();
    file_handle.read_to_string(&mut content).unwrap();
    HttpResponse::Ok().body(content)
}
