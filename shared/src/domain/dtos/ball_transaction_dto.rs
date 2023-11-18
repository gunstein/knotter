use serde::{Serialize, Deserialize};
use crate::domain::dtos::ball_dto::BallDto;

#[derive(Serialize, Deserialize)]
pub struct BallTransactionDto {
    pub transaction_id: String,
    pub ball_dto: BallDto,
}