use actix_web::{web, App, HttpResponse, HttpServer, Result, ResponseError, get, post, Error};
use scylla::{Session, SessionBuilder};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;
use std::fmt;
use scylla::IntoTypedRows;
use futures::StreamExt;


#[derive(Debug)]
pub enum CustomError {
    InternalServerError,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError")
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            },
        }
    }
}

#[derive(Deserialize)]
struct MyData {
    some_field: String,
    another_field: i32,
}

#[get("/{globe_id}/{transaction_id}")]
async fn get_data(
    path: web::Path<(String, Uuid)>,
    session: web::Data<Arc<Session>>,
) -> Result<HttpResponse, CustomError> {
    let globe_id = &path.0;
    let transaction_id = &path.1;

    let query = "SELECT transaction_id, operation_id, object_uuid, object_data FROM transactions WHERE globe_id = ? AND transaction_id = ?;";
    let values = (globe_id, transaction_id);

    let mut results = session
        .query_iter(query, values)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

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
            Err(_) => return Err(CustomError::InternalServerError),
        }
    }

    match response_data {
        Some((transaction_id, operation_id, object_uuid, object_data)) => Ok(HttpResponse::Ok().json(
            format!("Globe ID: {}, Transaction ID: {}, Operation ID: {}, Object UUID: {}, Object Data: {}",
                globe_id, transaction_id, operation_id, object_uuid, object_data
            )
        )),
        None => Err(CustomError::InternalServerError),
    }
}

#[post("/{globe_id}")]
async fn set_data(
    globe_id: web::Path<String>,
    session: web::Data<Arc<Session>>,
    data: web::Json<MyData>,
) -> Result<String, CustomError> {
    let new_transaction_id = Uuid::new_v4();
    let query = "INSERT INTO my_table (globe_id, transaction_id, some_field) VALUES (?, ?, ?)";
    let values = (globe_id.to_string(), new_transaction_id, data.some_field.clone());

    match session.query(query, values).await {
        Ok(_) => Ok(format!("Successfully inserted: Globe ID: {}, New Transaction ID: {}", globe_id, new_transaction_id)),
        Err(_) => Err(CustomError::InternalServerError),
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(session.clone()))
            .service(get_data)
            .service(set_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}