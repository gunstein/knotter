
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use nalgebra::Vector3;
use crate::domain::errors::my_error::MyError;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct InsertBallDto {
    pub is_fixed: bool,
    pub is_insert: bool,
    pub uuid: Uuid,
    pub color: Option<String>, 
    pub position: Option<InsertPositionDto>,
    pub impulse: Option<InsertImpulseDto>,
}

impl InsertBallDto {
    pub fn new(uuid: Uuid, is_insert: bool) -> Self {
        InsertBallDto {
            is_fixed: false,
            is_insert,
            uuid: uuid,
            color: None,
            position: None,
            impulse: None,
        }
    }

    pub fn validate_insert(&self) -> Result<(), MyError> {
        // Add validation logic here. 
        // For example, check if certain fields are present, 
        // if the values are within expected ranges, etc.

        // Example:
        if let Some(position) = &self.position {
            // validate position
        }

        // Return Ok if everything is valid
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InsertPositionDto {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl InsertPositionDto {
    pub fn distance_squared(&self, other: &InsertPositionDto) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx*dx + dy*dy + dz*dz
    }

    // Conversion to nalgebra::Vector3<f64>
    pub fn to_vector3(&self) -> Vector3<f64> {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InsertImpulseDto {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl InsertImpulseDto {
    // Conversion to nalgebra::Vector3<f64>
    pub fn to_vector3(&self) -> Vector3<f64> {
        Vector3::new(self.x, self.y, self.z)
    }
}

