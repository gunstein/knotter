use serde::Serialize;
use crate::domain::dtos::ball_transaction_dto::BallTransactionDto;

#[derive(Serialize)]
pub struct GetBallTransactionsByGlobeIdResponseDto {
    pub ball_transactions: Vec<BallTransactionDto>,
}