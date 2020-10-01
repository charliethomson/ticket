mod db;
mod routes;

use actix_web::{App, HttpServer};

const URL: &'static str = "localhost:8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use routes::*;
    let server = HttpServer::new(|| {
        App::new()
            .service(users_post)
            .service(users_get)
            .service(users_put)
            //
            .service(workorders_post)
            .service(workorders_get)
            .service(workorders_put)
            //
            .service(stores_put)
            .service(stores_post)
            .service(stores_get)
            //
            .service(notes_get)
            .service(notes_post)
            .service(notes_put)
            //
            .service(devices_put)
            .service(devices_post)
            .service(devices_get)
        // .service()
        // .service()
        // .service()
        // .service()
        // .service()
    })
    .bind(URL)?;
    println!("Listening on http://{}", URL);

    server.run().await
}
