mod errors;
mod serialization;
mod helpers;
mod point_validation;

use actix_web::{web, App, HttpResponse, HttpServer, Result, Responder, get, post, delete};
use uuid::Uuid;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use chrono::{self, Utc};
use redb::{Database, ReadableTable, TableDefinition};
use errors::MyError;
use serialization::*;
use helpers::*;

pub const TABLE: TableDefinition<&str, &str> = TableDefinition::new("knotter_log");


#[derive(Serialize)]
pub struct HealthResponse {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = HealthResponse {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}


// Define a structure to represent the data for clarity
#[derive(Serialize)]
struct TransactionDataOut {
    transaction_id: String,
    object_data: String,
}

#[get("/{globe_id}/{transaction_id}")]
async fn get_data_by_globe_id(
    path_info: web::Path<(String, String)>,
    db: web::Data<Arc<Database>>,
) -> Result<HttpResponse, MyError> {
    let (mut globe_id, transaction_id) = (path_info.0.clone(), path_info.1.clone());

    globe_id = process_globe_id(&globe_id)?;

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;

    let start = format!("{}--", globe_id);
    let end = format!("{}--{}", globe_id, "\u{10ffff}");
    let range = table.range::<&str>(start.as_str()..end.as_str()).unwrap();

    let mut results: Vec<_>;
    
    if transaction_id == "0" {
        results = range.collect();
    } else {
        results = range.rev().collect();
        results.reverse(); // To make the order correct
        results.truncate(results.len() - 1); // Skip the first (now last) item
    }

    let mut response_data = Vec::new();

    for item in results {
        match item {
            Ok((key, value)) => {
                let transaction_data = TransactionDataOut {
                    transaction_id: get_after_dashdash(key.value()).unwrap().to_string(),
                    object_data: value.value().to_string(),
                };
                response_data.push(transaction_data);
            },
            Err(err) => {
                return Err(MyError::DatabaseError(format!("Fetching of data failed: {}", err)));
            }
        }
    }

    Ok(HttpResponse::Ok().json(response_data))
}


#[post("/{globe_id}")]
async fn set_data(
    globe_id: web::Path<String>,
    db: web::Data<Arc<Database>>,
    data: web::Json<Transaction>,
) -> Result<String, MyError> {
    if let Transaction::Delete(_) = &*data {
        return Err(MyError::ValidationError("Only insert operations are allowed on this endpoint.".to_string()));
    }

    let transaction_data = if let Transaction::Insert(data) = &*data {
        data
    } else {
        // This block should never be hit given the previous check, but it's here for completeness.
        return Err(MyError::InternalServerError("Unexpected data type.".to_string()));
    };

    let globe_id = process_globe_id(&globe_id)?;

    transaction_data.validate(&globe_id, &*db)?;

    let serialized_data = serde_json::to_string(&*transaction_data)?; 

    let now = Utc::now();
    let nanoseconds_since_epoch = (now.timestamp_subsec_nanos() as i64 + now.timestamp() * 1_000_000_000).to_string();
    let key = format!("{}--{}", globe_id, nanoseconds_since_epoch);

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE)?;
        table.insert(&key.as_str(), &*serialized_data)?;
    }
    write_txn.commit()?;

    Ok(format!("Successfully inserted: Globe ID: {}, New Transaction ID: {}", globe_id, nanoseconds_since_epoch))
}

#[delete("/{globe_id}/{object_uuid}")]
async fn delete_data(
    path_info: web::Path<(String, Uuid)>,
    db: web::Data<Arc<Database>>,
    data: web::Json<Transaction>,
) -> Result<String, MyError> {
    let (globe_id, object_uuid) = path_info.into_inner();
    let globe_id = process_globe_id(&globe_id)?;

    if let Transaction::Insert(_) = &*data {
        return Err(MyError::ValidationError("Only delete operations are allowed on this endpoint.".to_string()));
    }

    let uuid_to_delete = if let Transaction::Delete(uuid) = &*data {
        uuid
    } else {
        // This block should never be hit given the previous check, but it's here for completeness.
        return Err(MyError::InternalServerError("Unexpected data type.".to_string()));
    };

    // Now you have both `globe_id` and `uuid_to_delete` to process the deletion
    // Make sure the provided UUID in the JSON body matches the one in the path, if necessary.
    if *uuid_to_delete != object_uuid {
        return Err(MyError::ValidationError("Mismatched UUIDs.".to_string()));
    }

    // Proceed with your deletion logic
    // ...

    Ok(format!("Successfully processed delete transaction for UUID: {}", uuid_to_delete))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = match Database::create("knotter_db.redb") {
        Ok(database) => database,
        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
    };

    let db = Arc::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(get_data_by_globe_id)
            .service(set_data)
            .service(healthcheck)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}