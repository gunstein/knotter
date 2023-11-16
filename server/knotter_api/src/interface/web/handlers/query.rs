use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use serde_json;
use crate::domain::errors::my_error::MyError;
use crate::domain::dtos::get_ball_transactions_by_globeid_response_dto::GetBallTransactionsByGlobeIdResponseDto;
use crate::helpers::*;
use actix_web::get;
use crate::infrastructure::database::key_value_store::KeyValueStore;
use crate::domain::dtos::ball_transaction_dto::BallTransactionDto;
use crate::domain::models::ball_entity::BallEntity;
use crate::domain::mapping::ball_mapper::entity_to_dto;
use log::debug;

#[get("/{globe_id}/{transaction_id}")]
async fn get_data_by_globe_id(
    path_info: web::Path<(String, String)>,
    key_value_store: web::Data<Arc<KeyValueStore>>,
) -> Result<HttpResponse, MyError> {
    let (globe_id, transaction_id) = (path_info.0.clone(), path_info.1.clone());
    //debug!("get_data_by_globe_id START: globe_id: {:?} transaction_id: {:?}", globe_id, transaction_id);

    let processed_globe_id = process_globe_id(&globe_id)?;

    let results = key_value_store.get_data(&processed_globe_id, &transaction_id)?;
    //debug!("results: {:?}", results);

    let ball_transactions: Vec<_> = results
        .into_iter()
        .map(|(key, value)| {
            // Deserialize the value string into a BallEntity
            //debug!("value: {:?}", value);
            let ball_entity: BallEntity = serde_json::from_str(&value)
                .map_err(|err| MyError::JsonError(err.to_string()))?;
            //debug!("ball_entity: {:?}", ball_entity);

            let ball_dto = entity_to_dto(&ball_entity);
            //debug!("ball_dto: {:?}", ball_dto);

            let transaction_id = get_after_dashdash(&key)
                .ok_or(MyError::ValidationError("Invalid transaction key format".to_string()))?;
            
            Ok(BallTransactionDto {
                transaction_id: transaction_id.to_string(),
                ball_dto,
            })
        })
        .collect::<Result<Vec<_>, MyError>>()?;  // Handle potential errors during mapping

    Ok(HttpResponse::Ok().json(GetBallTransactionsByGlobeIdResponseDto { ball_transactions }))
}
