use serde::Serialize;
use crate::domain::dtos::ball_dto::BallDto;

#[derive(Serialize)]
pub struct BallTransactionDto {
    pub transaction_id: String,
    pub ball_dto: BallDto,
}