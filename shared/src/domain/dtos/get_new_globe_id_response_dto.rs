use serde::Serialize;

#[derive(Serialize)]
pub struct GetNewGlobeIdResponse {
    pub new_globe_id: String,
}