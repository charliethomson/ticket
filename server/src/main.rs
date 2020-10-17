mod db;
mod routes;

use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{middleware::Logger, App, HttpServer};
const URL: &str = "localhost:8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web");
    env_logger::init();
    use routes::*;
    let server = HttpServer::new(|| {
        App::new()
            .data(AppState::new())
            .wrap(CookieSession::signed(&[0; 32]).name("offsite").secure(true))
            .wrap(Cors::new().send_wildcard().finish())
            .wrap(Logger::default())
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
            //
            .service(devices_put)
            .service(devices_post)
            .service(devices_get)
            //
            .service(customers_put)
            .service(customers_post)
            .service(customers_get)
            //
            .service(auth_login)
            .service(auth_response)
            .service(auth_me)
            //
            .service(front_end)
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind(URL)?;
    println!("Listening on http://{}", URL);

    server.run().await
}
