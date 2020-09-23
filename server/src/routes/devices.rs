use {
    crate::{
        db,
        models::{Customer, Device},
        routes::OkMessage,
    },
    actix_web::{get, post, web, HttpResponse, Responder},
    futures::stream::StreamExt,
    mongodb::{
        bson::{Bson, Document},
        error::Result as MongoResult,
        Cursor,
    },
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

#[post("/api/devices/new")]
async fn device_new(body: web::Json<DeviceNew>) -> impl Responder {
    let device_body = body.into_inner();
    let collection = db::get_collection("devices").await.unwrap();
    let device_count = collection.count_documents(None, None).await.unwrap();
    let device = Device {
        _id: device_count,
        serial: device_body.serial,
        name: device_body.name,
        customer: Customer::try_from_id(device_body.customer)
            .await
            .unwrap_or_default(),
        workorders: vec![],
        password: device_body.password,
    };
    let result = collection.insert_one(device.into_document(), None).await;
    match result {
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

#[get("/api/devices/find")]
async fn device_find(body: web::Json<DeviceFind>) -> impl Responder {
    let filter_in = body.into_inner();
    let mut filter_out = Document::new();
    if let Some(name) = filter_in.name {
        filter_out.insert("name", name);
    }
    if let Some(serial) = filter_in.serial {
        filter_out.insert("serial", serial);
    }
    // Customer attrs
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

    let collection = db::get_collection("devices").await.unwrap();
    let result = collection.find(Some(filter_out), None).await;
    match result {
        Ok(stream) => {
            let documents: Vec<MongoResult<Document>> = stream.collect().await;
            let devices: Vec<Device> = documents
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
