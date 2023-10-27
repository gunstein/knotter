mod application;
mod domain;
mod infrastructure;
mod interface;
mod helpers;

use actix_web::{web, App, HttpServer};
use std::env;
use std::sync::Arc;
use crate::interface::web::handlers::insert::handle_insert;
use crate::infrastructure::database::key_value_store::KeyValueStore;
use crate::application::services::validation_service::ValidationService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.contains(&"--test-mode".to_string());

    let db = KeyValueStore::setup_database(is_test_mode)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    // Here you create a single instance of KeyValueStore wrapped in an Arc
    let key_value_store = Arc::new(KeyValueStore::new(db));
    
    let validation_service = Arc::new(ValidationService::new());

    HttpServer::new(move || {
        App::new()
            // You share the same instance of KeyValueStore across requests
            .app_data(web::Data::new(key_value_store.clone()))
            .app_data(validation_service.clone())
            .service(handle_insert)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
