mod db;
mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{prelude::*, DateTime};
use futures::stream::StreamExt;
use models::*;
use mongodb::{
    bson::{self, document::Document, Bson, Deserializer},
    error::Result as MongoResult,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

const URL: &'static str = "localhost:8080";

#[derive(Serialize)]
pub struct ErrorResponse<Message: Serialize> {
    pub error: Message,
}

#[derive(Serialize)]
pub struct OkMessage<Message: Serialize> {
    pub ok: bool,
    pub message: Option<Message>,
}

#[get("/api/users/{id}")]
async fn get_user(web::Path(id): web::Path<u32>) -> impl Responder {
    let collection = db::get_collection("users").await.unwrap();
    let mut filter = Document::new();
    filter.insert("id", id);
    let stream = collection
        .find(Some(filter), None)
        .await
        .unwrap()
        .next()
        .await
        .unwrap()
        .unwrap();
    HttpResponse::Ok().json(User::deserialize(Deserializer::new(Bson::Document(stream))).unwrap())
}

#[get("/api/workorders/all")]
async fn workorders_all() -> impl Responder {
    let collection = db::get_collection("workorders").await.unwrap();
    let items_stream = collection.find(None, None).await.unwrap();
    let items_sync: Vec<MongoResult<Document>> = items_stream.collect().await;
    println!("{:?}", items_sync);
    HttpResponse::Ok().json(
        items_sync
            .iter()
            .filter(|res| res.is_ok())
            .cloned()
            .map(|res| Workorder::try_from(res.unwrap()).unwrap())
            .collect::<Vec<Workorder>>(),
    )
}

#[post("/api/users/new")]
async fn user_new(body: web::Json<User>) -> impl Responder {
    let new_user = body.into_inner();
    let document = new_user.into_document();
    if let Ok(collection) = db::get_collection("users").await {
        let result = collection.insert_one(document, None).await;
        if let Ok(ok) = result {
            println!("{:?}", ok);
            HttpResponse::Ok().json(OkMessage::<()> {
                ok: true,
                message: Some(()),
            })
        } else if let Err(e) = result {
            HttpResponse::NotAcceptable().json(OkMessage {
                ok: false,
                message: Some(format!("{}", e)),
            })
        } else {
            unreachable!()
        }
    } else {
        HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some("Failed to get collection `users`"),
        })
    }
}

#[get("/api/users/{id}")]
async fn user_get(web::Path(id): web::Path<i64>) -> impl Responder {
    let collection = db::get_collection("users").await.unwrap();
    let mut filter = Document::new();
    filter.insert("id", id);
    let user: Option<User> = match collection.find_one(Some(filter), None).await {
        Ok(Some(document)) => User::deserialize(Deserializer::new(Bson::Document(document))).ok(),
        _ => None,
    };

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NoContent().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(workorders_all)
            .service(user_get)
            .service(user_new)
    })
    .bind(URL)?;
    println!("Listening on http://{}", URL);

    server.run().await
}
