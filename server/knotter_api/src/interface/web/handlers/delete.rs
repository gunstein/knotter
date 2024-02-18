use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use uuid::Uuid;
use crate::domain::errors::my_error::MyError;
use crate::helpers::*;
use actix_web::delete;
use crate::application::services::validation_service::ValidationService;
use crate::infrastructure::database::key_value_store::KeyValueStore;
use crate::domain::models::ball_entity::BallEntity;
use log::debug;

#[delete("/{globe_id}/{object_uuid}")]
async fn delete_data(
    path_info: web::Path<(String, Uuid)>,
    key_value_store: web::Data<Arc<KeyValueStore>>,
) -> Result<HttpResponse, MyError> {
    let (globe_id, object_uuid) = path_info.into_inner();
    debug!("delete_data START. globe_id={}, object_uuid={:?}", globe_id, object_uuid);
    let globe_id = process_globe_id(&globe_id)?;

    ValidationService::validate_delete(&object_uuid, &globe_id, key_value_store.as_ref().as_ref())?;

    let delete_ball_entity = BallEntity::new(object_uuid, false);
    let serialized_data = serde_json::to_string(&delete_ball_entity)?; 

    debug!("Before key_value_store.delete. globe_id={}, serialized_data={:?}", globe_id, serialized_data);
    key_value_store.add_delete_to_log(&globe_id, &serialized_data)?;

    Ok(HttpResponse::Ok().body(format!("Successfully deleted: Globe ID: {}, Object_uuid: {}", globe_id, object_uuid)))
}
