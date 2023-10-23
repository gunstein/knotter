use actix_web::{web, HttpResponse, Result};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use redb::Database;
use crate::domain::errors::my_error::MyError;
use crate::domain::dtos::insert_ball_dto::InsertBallDto;
use crate::domain::dtos::insert_ball_response_dto::InsertBallResponseDto;
use crate::helpers::*;
use actix_web::{get, post, delete};
use crate::application::services::validation_service::ValidationService;
use crate::infrastructure::database::key_value_store::KeyValueStore;

#[post("/{globe_id}")]
pub async fn handle_insert(
    globe_id: web::Path<String>,
    db: web::Data<Arc<Database>>,
    repo: web::Data<Arc<KeyValueStore>>,
    data: web::Json<InsertBallDto>,
    validation_service: web::Data<Arc<ValidationService>>,
) -> Result<HttpResponse, MyError> {
    let globe_id = process_globe_id(&globe_id)?;

    validation_service.validate_insert(&data, &globe_id)?;

    let serialized_data = serde_json::to_string(&data)?; 

    let timestamp = generate_timestamp();
    repo.insert(&globe_id, &serialized_data, &timestamp)?;
    
    let response = InsertBallResponseDto {
        message: "Successfully inserted.".to_string(),
        globe_id,
        transaction_id: timestamp,
    };
    
    Ok(HttpResponse::Ok().json(response))
}