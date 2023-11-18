use serde::{Serialize, Deserialize};
use crate::domain::dtos::ball_transaction_dto::BallTransactionDto;

#[derive(Serialize, Deserialize)]
pub struct GetBallTransactionsByGlobeIdResponseDto {
    pub ball_transactions: Vec<BallTransactionDto>,
}