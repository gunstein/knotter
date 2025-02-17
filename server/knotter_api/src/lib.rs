// src/lib.rs
mod application;
pub mod domain;
mod infrastructure;
mod interface;
mod helpers;

// src/lib.rs

// ... existing module declarations ...
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use interface::web::handlers::query::get_new_globe_id;

use std::sync::Arc;
use crate::interface::web::handlers::delete::delete_data;
use crate::interface::web::handlers::health_check::healthcheck;
use crate::interface::web::handlers::insert::handle_insert;
//use crate::interface::web::handlers::insert::gvtest_insert;
use crate::interface::web::handlers::query::get_data_by_globe_id;
use crate::infrastructure::database::key_value_store::KeyValueStore;
use crate::application::services::validation_service::ValidationService;

pub async fn run_server(is_test_mode: bool) -> std::io::Result<()> {
    let db = KeyValueStore::setup_database(is_test_mode)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let key_value_store = Arc::new(KeyValueStore::new(db));
    let validation_service = Arc::new(ValidationService::new());

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(key_value_store.clone()))
            .app_data(web::Data::new(validation_service.clone()))
            .service(handle_insert)
            //.service(gvtest_insert)
            .service(delete_data)
            .service(healthcheck)
            .service(get_data_by_globe_id)
            .service(get_new_globe_id)
    })
    //.bind("127.0.0.1:8080")?
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
