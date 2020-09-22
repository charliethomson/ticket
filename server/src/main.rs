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

#[get("/api/users/{id}")]
async fn get_user(web::Path(id): web::Path<u32>) -> impl Responder {
    let collection = db::get_collection("users").await.unwrap();
    let mut filter = mongodb::bson::document::Document::new();
    filter.insert("id", id);
    let stream = collection
        .find(Some(filter), None)
        .await
        .unwrap()
        .next()
        .await
        .unwrap()
        .unwrap();
    HttpResponse::Ok().json(
        models::User::deserialize(mongodb::bson::Deserializer::new(
            mongodb::bson::Bson::Document(stream),
        ))
        .unwrap(),
    )
}

#[get("/api/workorders/all")]
async fn workorders_all() -> impl Responder {
    let collection = db::get_collection("workorders").await.unwrap();
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
