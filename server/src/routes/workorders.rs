use {
    crate::{db, models::*, routes::OkMessage},
    actix_web::{get, post, web, HttpResponse, Responder},
    chrono::{serde::ts_seconds, DateTime, Utc},
    futures::stream::StreamExt,
    mongodb::{bson::document::Document, error::Result as MongoResult},
    serde::{Deserialize, Serialize},
    std::convert::TryFrom,
};

#[derive(Serialize, Deserialize)]
pub struct WorkorderNew {
    pub origin: i64,
    pub travel_status: String,
    pub delivered_by: Option<String>,
    #[serde(with = "ts_seconds")]
    pub quoted_time: DateTime<Utc>,
    pub status: String,
    pub customer: i64, // Customer ID
    pub device: i64,   // Device ID
    pub brief: String,
    pub initial_note: InitialNote,
}

#[derive(Serialize, Deserialize)]
pub struct InitialNote {
    checked_in_by: i64, // User ID
    contents: String,
    #[serde(with = "ts_seconds")]
    next_update: DateTime<Utc>,
    location: Option<String>,
}


// API call to create and handle making a new workorder
#[post("/api/workorders/new")]
async fn workorder_new(body: web::Json<WorkorderNew>) -> impl Responder {
    //Get the workorders table
    let collection = db::get_collection("workorders").await.unwrap();
    let count = collection.count_documents(None, None).await.unwrap();
    let workorder = body.into_inner();
    // TODO: unwrap_or_default
    // Build the workorder
    let to_commit = Workorder {
        id: count,
        origin: Store::try_from_id(workorder.origin)
            .await
            .unwrap_or_default(),
        travel_status: workorder.travel_status,
        created: Utc::now(),
        quoted_time: workorder.quoted_time,
        status: workorder.status,
        customer: Customer::try_from_id(workorder.customer)
            .await
            .unwrap_or_default(),
        device: Device::try_from_id(workorder.device)
            .await
            .unwrap_or_default(),
        brief: workorder.brief,
        notes: vec![Note {
            id: db::get_collection("notes")
                .await
                .unwrap()
                .count_documents(None, None)
                .await
                .unwrap(),
            user: User::try_from_id(workorder.initial_note.checked_in_by)
                .await
                .unwrap_or_default(),
            created: Utc::now(),
            next_update: workorder.initial_note.next_update,
            contents: workorder.initial_note.contents,
            location: workorder.initial_note.location.unwrap_or(String::new()),
        }],
    };
    println!("{:#?}", to_commit);
    // Take created workorder and insert it into the workorders table
    let document: Document = to_commit.into_document();
    let insert_result = collection.insert_one(document, None).await;
    match insert_result {
        //Return the result
        Ok(_) => HttpResponse::Ok().json(OkMessage::<()> {
            ok: true,
            message: None,
        }),
        Err(e) => HttpResponse::NotModified().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

// API call to get all workorders in the table without any filtering
#[get("/api/workorders/all")]
async fn workorders_all() -> impl Responder {
    // Get all workorders from the table
    let collection = db::get_collection("workorders").await.unwrap();
    let items_stream = collection.find(None, None).await.unwrap();
    let items_sync: Vec<MongoResult<Document>> = items_stream.collect().await;
    println!("{:?}", items_sync);
    HttpResponse::Ok().json(
        items_sync
            .iter()
            .filter(|res| res.is_ok())
            .cloned()
            // Convert documents into workorder objects
            .map(|res| Workorder::try_from(res.unwrap()).unwrap())
            .collect::<Vec<Workorder>>(),
    )
}


// API call to get a workorder by its ID
#[get("/api/workorders/{id}")]
async fn workorder_by_id(web::Path(id): web::Path<i64>) -> impl Responder {
    // Get all workorders from the workorders table
    let collection = db::get_collection("workorders").await.unwrap();

    // Create the filter from the ID
    let mut filter = Document::new();
    filter.insert("_id", id);

    // Get the workorder using the filter
    let result = collection.find_one(Some(filter), None).await;
    match result {
        // TODO: Unwrap
        Ok(Some(workorder)) => HttpResponse::Ok().json(Workorder::try_from(workorder).unwrap()),
        _ => {
            eprintln!("{:#?}", result);
            HttpResponse::NotFound().finish()
        }
    }
}
