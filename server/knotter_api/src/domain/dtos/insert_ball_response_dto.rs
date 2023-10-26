use serde::Serialize;

#[derive(Serialize)]
pub struct InsertBallResponseDto {
    pub message: String,
    pub globe_id: String,
    pub transaction_id: String,
}