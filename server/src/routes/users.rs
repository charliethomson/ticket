use crate::{db, models::User, routes::OkMessage};
use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::bson::{document::Document, Bson, Deserializer};
use serde::Deserialize;

// API and function to create new user
#[post("/api/users/new")]
async fn user_new(body: web::Json<User>) -> impl Responder {
    let new_user = body.into_inner();
    let document = new_user.into_document();

    // Get the users table
    if let Ok(collection) = db::get_collection("users").await {
        // Attempt to insert the user into the users table
        let result = collection.insert_one(document, None).await;
        
        // Handle if there's an exception
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


// API and function to return a user by an ID
#[get("/api/users/{id}")]
async fn user_get(web::Path(id): web::Path<i64>) -> impl Responder {
    // Get the users table
    let collection = db::get_collection("users").await.unwrap();

    // Create a filter using the ID passed to the API
    let mut filter = Document::new();
    filter.insert("id", id);

    // Attempt to find the user using the filter
    let user: Option<User> = match collection.find_one(Some(filter), None).await {
        Ok(Some(document)) => User::deserialize(Deserializer::new(Bson::Document(document))).ok(),
        _ => None,
    };

    // Return the result
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NoContent().finish(),
    }
}
