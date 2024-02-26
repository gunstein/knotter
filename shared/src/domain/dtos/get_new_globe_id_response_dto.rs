use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetNewGlobeIdResponse {
    pub new_globe_id: String,
}