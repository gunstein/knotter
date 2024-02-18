use actix_web::{web, HttpResponse, Result};
use actix_web::web::Bytes;
use std::sync::Arc;
use crate::domain::errors::my_error::MyError;
use shared::domain::dtos::insert_ball_dto::InsertBallDto;
use shared::domain::dtos::insert_ball_response_dto::InsertBallResponseDto;
use crate::domain::mapping::ball_mapper::dto_to_entity;
use crate::helpers::*;
use actix_web::post;
use crate::application::services::validation_service::ValidationService;
use crate::infrastructure::database::key_value_store::KeyValueStore;
use log::debug;
/* 
#[post("/{globe_id}")]
async fn gvtest_insert(item: Bytes) -> Result<String, MyError> {
    // Convert Bytes to String
    let json_str = String::from_utf8(item.to_vec()).unwrap();

    // Log the raw JSON
    println!("Raw JSON Payload: {}", json_str);

    // Deserialize manually
    let deserialized: Result<InsertBallDto, _> = serde_json::from_str(&json_str);

    match deserialized {
        Ok(dto) => {
            // Process your deserialized DTO
            Ok(format!("Welcome {}!", dto.uuid))
        }
        Err(err) => {
            // Handle deserialization error
            println!("Deserialization error: {:?}", err);
            Err(MyError::JsonError("Invalid JSON".to_string()))
        }
    }
}
*/

#[post("/{globe_id}")]
pub async fn handle_insert(
    globe_id: web::Path<String>,
    key_value_store: web::Data<Arc<KeyValueStore>>,
    data: web::Json<InsertBallDto>,
    validation_service: web::Data<Arc<ValidationService>>,
) -> Result<HttpResponse, MyError> {
    debug!("handle_insert START. globe_id={}, data={:?}", globe_id, data);
    let globe_id = process_globe_id(&globe_id)?;
    debug!("handle_insert 1");
    let insert_ball_dto: InsertBallDto = data.into_inner();
    debug!("insert_ball_dto {:?}", insert_ball_dto);
    debug!("handle_insert 2");
    let ball_entity = dto_to_entity(&insert_ball_dto);
    debug!("ball_entity {:?}", ball_entity);
    debug!("handle_insert 3");
    validation_service.validate_insert(&ball_entity, &globe_id, key_value_store.as_ref().as_ref())?;
    debug!("handle_insert 4");
    let serialized_data = serde_json::to_string(&ball_entity)?; 
    debug!("handle_insert 5");
    let timestamp = generate_timestamp();
    key_value_store.add_insert_to_log(&globe_id, &serialized_data, &timestamp)?;
    debug!("handle_insert 6");
    let response = InsertBallResponseDto {
        message: "Successfully inserted.".to_string(),
        globe_id,
        transaction_id: timestamp,
    };
    
    debug!("handle_insert response: {:?}", response);

    Ok(HttpResponse::Ok().json(response))
}