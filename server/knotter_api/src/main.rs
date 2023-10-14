use actix_web::{web, App, HttpResponse, HttpServer, Result, ResponseError, Responder, get, post};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::fmt;
use chrono::{Utc};
use redb::{Database, ReadableTable, TableDefinition};

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("knotter_log");

#[derive(Debug)]
enum MyError {
    NotFound,
    DatabaseError(String),
    ValidationError(String),
    InternalServerError(String),
    // ... other errors
}

impl From<redb::TransactionError> for MyError {
    fn from(err: redb::TransactionError) -> Self {
        MyError::DatabaseError(err.to_string())
    }
}

impl From<redb::TableError> for MyError {
    fn from(err: redb::TableError) -> Self {
        MyError::DatabaseError(err.to_string())
    }
}

impl From<redb::StorageError> for MyError {
    fn from(err: redb::StorageError) -> Self {
        MyError::DatabaseError(err.to_string())
    }
}

impl From<redb::CommitError> for MyError {
    fn from(err: redb::CommitError) -> Self {
        MyError::DatabaseError(err.to_string())
    }
}


impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::NotFound => write!(f, "Not found"),
            MyError::DatabaseError(ref message) => write!(f, "Database error: {}", message),
            MyError::ValidationError(ref message) => write!(f, "Validation error: {}", message),
            MyError::InternalServerError(ref message) => write!(f, "Internal error: {}", message),
            // ... other error variants
        }
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::DatabaseError(ref message) => HttpResponse::InternalServerError().json(message),
            MyError::ValidationError(ref message) => HttpResponse::BadRequest().json(message),
            MyError::InternalServerError(ref message) => HttpResponse::InternalServerError().json(message),
            // ... other error mappings
        }
    }
}

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
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

fn get_after_dashdash(s: &str) -> Option<&str> {
    let mut parts = s.split("--");
    parts.next()?;  // consume everything before "--"
    parts.next()    // get everything after "--"
}

#[get("/{globe_id}/{transaction_id}")]
async fn get_data_by_globe_id(
    path_info: web::Path<(String, String)>,
    db: web::Data<Arc<Database>>,
) -> Result<HttpResponse, MyError> {
    let globe_id = &path_info.0;
    let transaction_id = &path_info.1;

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;

    let mut iter = {
        let start = if transaction_id == "0" {
            format!("{}--", globe_id)
        } else {
            format!("{}--{}", globe_id, transaction_id)
        };

        let end = format!("{}--{}", globe_id, "\u{10ffff}");
        table.range::<&str>(start.as_str()..end.as_str()).unwrap()
    };

    if transaction_id != "0" {
        // Skip the first item
        iter.next();
    }

    let mut response_data = Vec::new();
    for item in iter {
        match item {
            Ok((key, value)) => {
                let transaction_data = TransactionDataOut {
                    transaction_id: get_after_dashdash(key.value()).unwrap().to_string(),
                    object_data: value.value().to_string(),
                };
                response_data.push(transaction_data);
            },
            Err(err) => {
                return Err(MyError::DatabaseError(format!("Fetching of data failed: {}", err)))
            }
        }
    }

    Ok(HttpResponse::Ok().json(response_data))
}

#[derive(Deserialize)]
struct TransactionDataIncoming {
    object_data: String,
}

#[post("/{globe_id}")]
async fn set_data(
    globe_id: web::Path<String>,
    db: web::Data<Arc<Database>>,
    data: web::Json<TransactionDataIncoming>,
) -> Result<String, MyError> {
    //Must validate first

    let now = Utc::now();
    let nanoseconds_since_epoch = (now.timestamp_subsec_nanos() as i64 + now.timestamp() * 1_000_000_000).to_string();
    let key = format!("{}--{}", globe_id, nanoseconds_since_epoch);

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE)?;
        table.insert(&key.as_str(), &data.object_data.as_str())?;
    }
    write_txn.commit()?;

    Ok(format!("Successfully inserted: Globe ID: {}, New Transaction ID: {}", globe_id, nanoseconds_since_epoch))
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