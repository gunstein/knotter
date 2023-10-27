use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PositionDto {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}