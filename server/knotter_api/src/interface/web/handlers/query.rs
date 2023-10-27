use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use crate::domain::errors::my_error::MyError;
use crate::domain::dtos::get_ball_transactions_by_globeid_response_dto::GetBallTransactionsByGlobeIdResponseDto;
use crate::helpers::*;
use actix_web::get;
use crate::application::services::validation_service::ValidationService;
use crate::infrastructure::database::key_value_store::KeyValueStore;
use crate::domain::dtos::ball_transaction_dto::BallTransactionDto;
use crate::domain::dtos::ball_dto::BallDto;
use serde_json;

#[get("/{globe_id}/{transaction_id}")]
async fn get_data_by_globe_id(
    path_info: web::Path<(String, String)>,
    key_value_store: web::Data<Arc<KeyValueStore>>,
) -> Result<HttpResponse, MyError> {
    let (globe_id, transaction_id) = (path_info.0.clone(), path_info.1.clone());

    let processed_globe_id = process_globe_id(&globe_id)?;

    let results = key_value_store.get_data(&processed_globe_id, &transaction_id)?;

    let ball_transactions: Vec<_> = results
        .into_iter()
        .filter_map(|(key, value)| {
            let transaction_id = get_after_dashdash(&key).unwrap().to_string();
            // Deserialize the JSON value into a BallDto
            match serde_json::from_str::<BallDto>(&value) {
                Ok(ball) => Some(BallTransactionDto { transaction_id, ball }),
                Err(_) => None
            }
        })
        .collect();

    let response_data = GetBallTransactionsByGlobeIdResponseDto {
        ball_transactions
    };

    Ok(HttpResponse::Ok().json(response_data))
}
