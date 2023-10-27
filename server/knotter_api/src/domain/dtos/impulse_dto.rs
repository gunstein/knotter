use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImpulseDto {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}