use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InsertBallResponseDto {
    pub message: String,
    pub globe_id: String,
    pub transaction_id: String,
}