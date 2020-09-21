use mongodb::{error::Result as MongoResult, Client, Collection, Database};

const DB_URI: &'static str = "mongodb://localhost:27017";

pub async fn get_collection<S: ToString>(
    database: S,
    collection_name: S,
) -> MongoResult<Collection> {
    let client = Client::with_uri_str(DB_URI).await?;
    Ok(client
        .database(&database.to_string())
        .collection(&collection_name.to_string()))
}
