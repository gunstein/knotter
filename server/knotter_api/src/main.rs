use actix_web::{web, App, HttpResponse, HttpServer, Result, ResponseError, Responder, get, post, Error};
use scylla::{Session, SessionBuilder};
use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::fmt;
use scylla::IntoTypedRows;
use futures::StreamExt;
use chrono::{Utc, Local, DateTime};



#[derive(Debug)]
enum MyError {
    NotFound,
    DatabaseError(String),
    ValidationError(String),
    InternalServerError(String),
    // ... other errors
}

/* 
#[derive(Debug)]
pub enum CustomError {
    InternalServerError,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError")
    }
}
*/

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

/* 
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            },
        }
    }
}
*/

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





#[get("/{globe_id}/{transaction_id}")]
async fn get_data(
    path: web::Path<(String, Uuid)>,
    session: web::Data<Arc<Session>>,
) -> Result<HttpResponse, MyError> {
    let globe_id = &path.0;
    let transaction_id = &path.1;

    let query = "SELECT transaction_id, operation_id, object_uuid, object_data FROM transactions WHERE globe_id = ? AND transaction_id = ?;";
    let values = (globe_id, transaction_id);


    //return Err(MyError::ValidationError("Invalid input".to_string()));
    let mut results = session
        .query_iter(query, values)
        .await
        .map_err(|_| MyError::DatabaseError("Execution of query failed".to_string()))?;

    let mut response_data = None;

    while let Some(row_result) = results.next().await {
        match row_result {
            Ok(row) => {
                let transaction_id: Uuid = row.columns[0].as_ref().unwrap().as_uuid().unwrap();
                let operation_id: i32 = row.columns[1].as_ref().unwrap().as_int().unwrap() as i32;
                let object_uuid: Uuid = row.columns[2].as_ref().unwrap().as_uuid().unwrap();
                let object_data: String = row.columns[3].as_ref().unwrap().as_text().unwrap().to_string();

                response_data = Some((
                    transaction_id,
                    operation_id,
                    object_uuid,
                    object_data,
                ));
            },
            Err(err) => return Err(MyError::DatabaseError("Fetching of data failed: ".to_string() + err.to_string().as_str())),
        }
    }

    match response_data {
        Some((transaction_id, operation_id, object_uuid, object_data)) => Ok(HttpResponse::Ok().json(
            format!("Globe ID: {}, Transaction ID: {}, Operation ID: {}, Object UUID: {}, Object Data: {}",
                globe_id, transaction_id, operation_id, object_uuid, object_data
            )
        )),
        None => Err(MyError::InternalServerError("response_data is None.".to_string())),
    }
}


// Define a structure to represent the data for clarity
#[derive(Serialize)]
struct TransactionDataOut {
    transaction_id: Uuid,
    operation_id: i32,
    object_uuid: Uuid,
    object_data: String,
}

#[get("/{globe_id}")]
async fn get_data_by_globe_id(
    globe_id: web::Path<String>,
    session: web::Data<Arc<Session>>,
) -> Result<HttpResponse, MyError> {

    let query = "SELECT transaction_id, operation_id, object_uuid, object_data FROM mykeyspace.transactions WHERE globe_id = ?;";
    let values = (globe_id.to_string(),);

    let mut results = session
        .query_iter(query, values)
        .await
        .map_err(|err| MyError::DatabaseError(format!("Execution of query failed: {}", err)))?;

    let mut response_data = Vec::new();

    while let Some(row_result) = results.next().await {
        match row_result {
            Ok(row) => {
                let transaction_data = TransactionDataOut {
                    transaction_id: row.columns[0].as_ref().and_then(|col| col.as_uuid()).ok_or(MyError::InternalServerError("Invalid transaction_id".into()))?,
                    operation_id: row.columns[1].as_ref().and_then(|col| col.as_int()).ok_or(MyError::InternalServerError("Invalid operation_id".into()))? as i32,
                    object_uuid: row.columns[2].as_ref().and_then(|col| col.as_uuid()).ok_or(MyError::InternalServerError("Invalid object_uuid".into()))?,
                    object_data: row.columns[3].as_ref().and_then(|col| col.as_text()).ok_or(MyError::InternalServerError("Invalid object_data".into()))?.to_string(),
                };
                response_data.push(transaction_data);
            },
            Err(err) => return Err(MyError::DatabaseError(format!("Fetching of data failed: {}", err))),
        }
    }

    Ok(HttpResponse::Ok().json(response_data))
}
/* 
#[get("/{globe_id}")]
async fn get_data_by_globe_id(
    globe_id: web::Path<String>,
    session: web::Data<Arc<Session>>,
) -> Result<HttpResponse, MyError> {

    // Query to select all columns based on globe_id
    let query = "SELECT transaction_id, operation_id, object_uuid, object_data FROM mykeyspace.transactions WHERE globe_id = ?;";
    let values = (globe_id.to_string(),);

    let mut results = session
        .query_iter(query, values)
        .await
        .map_err(|err| MyError::DatabaseError("Execution of query failed: ".to_string() + &err.to_string().as_str()))?;

    let mut response_data = Vec::new();

    while let Some(row_result) = results.next().await {
        match row_result {
            Ok(row) => {
                let transaction_id: Uuid = row.columns[0].as_ref().unwrap().as_uuid().unwrap();
                let operation_id: i32 = row.columns[1].as_ref().unwrap().as_int().unwrap() as i32;
                let object_uuid: Uuid = row.columns[2].as_ref().unwrap().as_uuid().unwrap();
                let object_data: String = row.columns[3].as_ref().unwrap().as_text().unwrap().to_string();

                // Storing fetched data in a tuple and pushing it into response_data vector
                response_data.push((
                    transaction_id,
                    operation_id,
                    object_uuid,
                    object_data,
                ));
            },
            Err(err) => return Err(MyError::DatabaseError("Fetching of data failed: ".to_string() + err.to_string().as_str())),
        }
    }

    // Return as JSON
    Ok(HttpResponse::Ok().json(response_data))
}
*/
#[derive(Deserialize)]
struct TransactionDataIncoming {
    operation_id: i32,
    object_uuid: Uuid,
    object_data: String,
}

#[post("/{globe_id}")]
async fn set_data(
    globe_id: web::Path<String>,
    session: web::Data<Arc<Session>>,
    data: web::Json<TransactionDataIncoming>,
) -> Result<String, MyError> {
//) -> HttpResponse {
    //println!("START");
    // MAC address for demonstration; replace with your own or another method to get a node ID.
    let node_id: [u8; 6] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];

    let new_transaction_id = Uuid::now_v1(&node_id);
    //println!("GV1");
    let query = "INSERT INTO mykeyspace.transactions(globe_id, transaction_id, operation_id, object_uuid, object_data) VALUES (?, ?, ?, ?, ?)";
    let values = (globe_id.to_string(), new_transaction_id, data.operation_id, data.object_uuid, data.object_data.clone());
    //println!("GV2");
    match session.query(query, values).await {
        Ok(_) => Ok(format!("Successfully inserted: Globe ID: {}, New Transaction ID: {}", globe_id, new_transaction_id)),
        //Ok(todo) => HttpResponse::Ok().json(query),
        //Err(_) => Err(CustomError::InternalServerError),
        Err(err) => Err(MyError::DatabaseError("Insert failed: ".to_string() + &err.to_string().as_str())),
        //Err(err) => CustomError::InternalServerError().body(err.to_string()),
        //Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    let session = SessionBuilder::new()
        .known_node(uri)
        .build()
        .await
        .expect("Session building failed");
    
    let session = Arc::new(session);

    session.query("CREATE KEYSPACE IF NOT EXISTS mykeyspace WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}", &[]).await;

    session
    .query(
        "CREATE TABLE IF NOT EXISTS mykeyspace.transactions (
            globe_id text,
            transaction_id TIMEUUID,
            operation_id int,
            object_uuid UUID,
            object_data text,
            PRIMARY KEY (globe_id, transaction_id)
        )",
        &[],
    )
    .await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(session.clone()))
            .service(get_data)
            .service(get_data_by_globe_id)
            .service(set_data)
            .service(healthcheck)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}