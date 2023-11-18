use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImpulseDto {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}