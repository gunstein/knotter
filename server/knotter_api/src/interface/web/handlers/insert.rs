use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use crate::domain::errors::my_error::MyError;
use crate::domain::dtos::insert_ball_dto::InsertBallDto;
use crate::domain::dtos::insert_ball_response_dto::InsertBallResponseDto;
use crate::domain::mapping::ball_mapper::dto_to_entity;
use crate::helpers::*;
use actix_web::post;
use crate::application::services::validation_service::ValidationService;
use crate::infrastructure::database::key_value_store::KeyValueStore;

#[post("/{globe_id}")]
pub async fn handle_insert(
    globe_id: web::Path<String>,
    key_value_store: web::Data<Arc<KeyValueStore>>,
    data: web::Json<InsertBallDto>,
    validation_service: web::Data<Arc<ValidationService>>,
) -> Result<HttpResponse, MyError> {
    let globe_id = process_globe_id(&globe_id)?;

    let insert_ball_dto: InsertBallDto = data.into_inner();

    let ball_entity = dto_to_entity(&insert_ball_dto);
    //validation_service.validate_insert(&ball_entity, &globe_id, &key_value_store)?;
    validation_service.validate_insert(&ball_entity, &globe_id, key_value_store.as_ref().as_ref())?;


    let serialized_data = serde_json::to_string(&ball_entity)?; 

    let timestamp = generate_timestamp();
    key_value_store.insert(&globe_id, &serialized_data, &timestamp)?;
    
    let response = InsertBallResponseDto {
        message: "Successfully inserted.".to_string(),
        globe_id,
        transaction_id: timestamp,
    };
    
    Ok(HttpResponse::Ok().json(response))
}