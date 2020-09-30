mod db;
mod routes;

use actix_web::{App, HttpServer};

const URL: &'static str = "localhost:8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use routes::*;
    let server = HttpServer::new(|| {
        App::new()
            .service(workorders_all)
            .service(workorders_find)
            .service(user_get)
            // .service(user_new)
            .service(workorder_new)
            .service(workorder_by_id)
        // .service(device_find)
        // .service(device_new)
    })
    .bind(URL)?;
    println!("Listening on http://{}", URL);

    server.run().await
}
