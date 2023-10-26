use nalgebra::Vector3;
use crate::domain::models::ball_entity::{PositionEntity, ImpulseEntity};
use crate::domain::errors::my_error::MyError;

const TOLERANCE: f64 = 1e-6;
const MIN_IMPULSE_MAGNITUDE: f64 = 0.0; 
const MAX_IMPULSE_MAGNITUDE: f64 = 1.0;

pub fn validate_impulse_direction(position: &PositionEntity, impulse: &ImpulseEntity) -> Result<(), MyError> {
    let dir_from_center = position.to_vector3().normalize();
    let impulse_direction = impulse.to_vector3().normalize();

    // dot product using nalgebra
    let dot = dir_from_center.dot(&impulse_direction);

    if dot.abs() > TOLERANCE {
        return Err(MyError::ValidationError("Impulse direction is not tangential to the globe's surface.".to_string()));
    }

    Ok(())
}

pub fn validate_impulse_magnitude(impulse: &ImpulseEntity) -> Result<(), MyError> {
    // magnitude computation using nalgebra
    let impulse_magnitude = impulse.to_vector3().magnitude();
    if impulse_magnitude < MIN_IMPULSE_MAGNITUDE || impulse_magnitude > MAX_IMPULSE_MAGNITUDE {
        return Err(MyError::ValidationError("Impulse magnitude is out of acceptable bounds.".to_string()));
    }

    Ok(())
}