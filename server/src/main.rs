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

#[get("/")]
async fn index() -> impl Responder {
    ""
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

// R/W/R TESTING
// async fn main() {
//     let workorder = models::Workorder {
//         id: 0,
//         origin: models::Store {
//             id: 0,
//             name: String::from("test store"),
//             contact_name: String::from("test user"),
//             phone_number: String::from("5555551234"),
//             workorders: vec![],
//         },
//         travel_status: String::from("Delivered"),
//         delivered_by: String::from("test user"),
//         created: Utc::now(),
//         last_update: Utc::now(),
//         quoted_time: Utc::now(),
//         status: 0,
//         customer: models::Customer {
//             id: 0,
//             name: String::from("test customer"),
//             phone_number: String::from("5555554321"),
//             email: String::from("decline@customer.com"),
//             devices: vec![],
//             workorders: vec![],
//         },
//         device: models::Device {
//             id: 0,
//             serial: String::from("123456789AAA"),
//             customer: models::Customer {
//                 id: 0,
//                 name: String::from("test customer"),
//                 phone_number: String::from("5555554321"),
//                 email: String::from("decline@customer.com"),
//                 devices: vec![],
//                 workorders: vec![],
//             },
//             workorders: vec![],
//         },
//         notes: vec![models::Note {
//             id: 0,
//             user: models::User {
//                 id: 0,
//                 name: String::from("test employee"),
//                 phone_number: String::from("5555551212"),
//                 queue: vec![],
//             },
//             created: Utc::now(),
//             next_update: Utc::now(), // TODO ?
//             contents: String::from("test note 1"),
//             location: String::from(""),
//         }],
//     };
//     println!("\n\nIN \n{:#?}\n\n", workorder);
//     let doc = match workorder.into_bson() {
//         mongodb::bson::Bson::Document(doc) => doc,
//         _ => panic!(),
//     };
//     let collection = db::get_collection("workorders").await.unwrap();
//     println!("\n\tResult: {:?}", collection.insert_one(doc, None).await);
//     let out_wo = collection
//         .find(None, None)
//         .await
//         .unwrap()
//         .collect::<Vec<mongodb::error::Result<mongodb::bson::document::Document>>>()
//         .await
//         .iter()
//         .filter(|res| res.is_ok())
//         .cloned()
//         .map(|res| models::Workorder::from(res.unwrap()))
//         .collect::<Vec<models::Workorder>>();
//     println!("\n\nOUT \n{:#?}\n\n", out_wo);
//     let mut doc = mongodb::bson::document::Document::new();
//     doc.insert("delivered_by", "test user");
//     println!("REMOVING: {:#?}", collection.delete_many(doc, None).await);
// }
