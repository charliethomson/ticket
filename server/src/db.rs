use mongodb::{error::Result as MongoResult, Client, Collection, Database};

const DB_URI: &'static str = "mongodb://charliecthomson.com:27020";

pub async fn get_collection<S: ToString>(collection_name: S) -> MongoResult<Collection> {
    let client = Client::with_uri_str(DB_URI).await?;
    Ok(client
        .database("offsite")
        .collection(&collection_name.to_string()))
}
