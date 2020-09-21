use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

mod db;
mod models;

#[derive(Serialize)]
pub struct JsonResponse<Data: Serialize> {
    data: Data,
}

const URL: &'static str = "localhost:8080";

#[get("/")]
async fn index() -> impl Responder {
    ""
}

#[get("/api/workorders/all")]
async fn workorders_all() -> impl Responder {
    let collection = db::get_collection("offsite", "workorders").await.unwrap();
    let items_stream = collection.find(None, None).await.unwrap();
    let items_sync: Vec<mongodb::error::Result<mongodb::bson::document::Document>> =
        items_stream.collect().await;
    println!("{:?}", items_sync);
    HttpResponse::Ok().json(
        items_sync
            .iter()
            .filter(|res| res.is_ok())
            .map(|res| res.clone().unwrap())
            .collect::<Vec<mongodb::bson::document::Document>>(),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(workorders_all)).bind(URL)?;
    println!("Listening on http://{}", URL);

    server.run().await
}
