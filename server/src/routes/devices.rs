use {
    crate::{
        db,
        models::{Customer, Device},
        routes::OkMessage,
    },
    actix_web::{get, post, web, HttpResponse, Responder},
    futures::stream::StreamExt,
    mongodb::{bson::Document, error::Result as MongoResult},
    serde::{Deserialize, Serialize},
    std::convert::TryFrom,
};

#[derive(Serialize, Deserialize)]
pub struct DeviceNew {
    pub customer: i64,
    pub name: String,
    pub serial: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceFind {
    pub customer_attributes: Option<CustomerAttributes>,
    pub name: Option<String>,
    pub serial: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomerAttributes {
    pub id: Option<i64>,
    pub phone_number: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}


//API call to create a new device, server listening on "/api/devices/new"
#[post("/api/devices/new")]
async fn device_new(body: web::Json<DeviceNew>) -> impl Responder {
    //Get JSON from POST and convert into device struct
    let device_body = body.into_inner();
    let collection = db::get_collection("devices").await.unwrap();
    let device_count = collection.count_documents(None, None).await.unwrap();
    let device = Device {
        id: device_count,
        serial: device_body.serial,
        name: device_body.name,
        customer: Customer::try_from_id(device_body.customer)
            .await
            .unwrap_or_default(),
        workorders: vec![],
        password: device_body.password,
    };

    //From this struct we insert the device into the database
    let result = collection.insert_one(device.into_document(), None).await;
    match result {
        //Return with a response of database insertion
        Ok(inserted) => HttpResponse::Ok().json(OkMessage {
            ok: true,
            message: Some(inserted.inserted_id),
        }),
        Err(e) => {
            eprintln!("{}", e.to_string());
            HttpResponse::InternalServerError().json(OkMessage {
                ok: false,
                message: Some(e.to_string()),
            })
        }
    }
}


//API call for finding a device in the database
#[get("/api/devices/find")]
async fn device_find(body: web::Json<DeviceFind>) -> impl Responder {
    let filter_in = body.into_inner();
    let mut filter_out = Document::new();

    //Constructing a filter for the database:

    //Check if name exists
    if let Some(name) = filter_in.name {
        filter_out.insert("name", name);
    }
    
    //Check if serial exists:
    if let Some(serial) = filter_in.serial {
        filter_out.insert("serial", serial);
    }

    // Check if customer attributes exists:
    if let Some(customer_attrs) = filter_in.customer_attributes {
        let mut customer_filter = Document::new();
        if let Some(phone_number) = customer_attrs.phone_number {
            customer_filter.insert("phone_number", phone_number);
        }
        if let Some(email) = customer_attrs.email {
            customer_filter.insert("email", email);
        }
        if let Some(name) = customer_attrs.name {
            customer_filter.insert("name", name);
        }
        if let Some(id) = customer_attrs.id {
            customer_filter.insert("_id", id);
        }
        filter_out.insert("customer", customer_filter);
    }

    //Grab the devices table from the database the database and find the device using the filter
    let collection = db::get_collection("devices").await.unwrap();
    let result = collection.find(Some(filter_out), None).await;
    match result {
        //Checking the stream of devices from the table
        Ok(stream) => {
            let documents: Vec<MongoResult<Document>> = stream.collect().await;
            let devices: Vec<Device> = documents
                //iterate over the vec of all documents that passed the filter, converts each into a device, and then add them to a vec, and returns the result
                .iter()
                .filter(|res| res.is_ok())
                .cloned()
                .map(|res| res.unwrap())
                .map(|doc| Device::try_from(doc))
                // TODO
                .filter(|res| res.is_ok())
                .map(|res| res.unwrap())
                .collect();

            HttpResponse::Ok().json(OkMessage {
                ok: true,
                message: Some(devices),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(OkMessage {
            ok: false,
            message: Some(e.to_string()),
        }),
    }
}

#[get("/api/devices/{id}")]
async fn device_id(web::Path(id): web::Path<i64>) -> impl Responder {
    let collection = db::get_collection("devices").await.unwrap();
    let mut filter = Document::new();
    filter.insert("_id", id);

    let result = collection.find_one(Some(filter), None).await;
    match result {
        Ok(device) => "",
        Err(e) => {
            eprintln!("{}", e.to_string());
            ""
        }
    }
}
