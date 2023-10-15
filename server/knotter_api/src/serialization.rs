use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::{MyError};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionDataIncoming {
    is_fixed: bool,
    is_insert: bool,
    object_uuid: Uuid,
    color: String,
    position: Position,
    velocity: Option<Velocity>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Velocity {
    v_x: f64,
    v_y: f64,
    v_z: f64,
}

impl TransactionDataIncoming {
    pub fn validate(&self) -> Result<(), MyError> {
        if self.is_fixed && self.velocity.is_some() {
            return Err(MyError::ValidationError("Velocity should be None for fixed objects.".to_string()));
        }
        // You can add more validations here as needed
        Ok(())
    }
}