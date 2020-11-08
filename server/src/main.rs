mod db;
// mod handlers;
mod macros;
mod routes;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware::Logger, App, HttpServer};
// use handlers::OffsiteHandler;
use rand::prelude::Rng;
const URL: &str = "localhost:8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web");
    env_logger::init();
    use routes::*;
    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    let server = HttpServer::new(move || {
        App::new()
            .data(AppState::new())
            // Middleware
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("offsite")
                    .secure(false),
            ))
            .wrap(
                Cors::default()
                    // .allow_any_origin()
                    .allowed_origin("*")
                    // .allowed_origin("https://portal.ubif.net")
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            // .wrap(OffsiteHandler::new())
            .wrap(Logger::default())
            // Services
            // users
            .service(users_post)
            .service(users_get)
            .service(users_put)
            // workorders
            .service(workorders_post)
            .service(workorders_get)
            .service(workorders_put)
            // stores
            .service(stores_put)
            .service(stores_post)
            .service(stores_get)
            // notes
            .service(notes_get)
            .service(notes_post)
            // devices
            .service(devices_put)
            .service(devices_post)
            .service(devices_get)
            // customers
            .service(customers_put)
            .service(customers_post)
            .service(customers_get)
            // auth
            .service(auth_login)
            .service(auth_response)
            .service(auth_me)
            // staticfiles
            .service(front_end)
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind(URL)?;
    println!("Listening on http://{}", URL);

    server.run().await
}
