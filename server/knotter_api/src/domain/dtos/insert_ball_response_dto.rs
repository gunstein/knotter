use serde::Serialize;

#[derive(Serialize)]
pub struct InsertBallResponseDto {
    message: String,
    globe_id: String,
    transaction_id: String,
}