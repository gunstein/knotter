use crate::{
    TransactionData, MyError, Ball, Globe, Position, 
    is_valid_distance_from_others, validate_color, Transaction
};
use redb::Database;

pub fn validate_insert(data: &TransactionData, globe_id: &str, db: &Database) -> Result<(), MyError> {
    // Preliminary checks
    if data.is_fixed && *data.impulse.is_some() {
        return Err(MyError::ValidationError("Velocity should be None for fixed objects.".to_string()));
    }

    // Retrieve all alive objects
    let map_alive_objects = Transaction::get_alive_objects_map(globe_id, db)?;

    let mut vec_position_alive_fixed_objects: Vec<&Position> = Vec::new();
    for value in map_alive_objects.values() {
        if value.is_fixed {
            if let Some(position) = &value.position {
                vec_position_alive_fixed_objects.push(position);
            }
        }
    }

    // Check that the new object is on the surface of the sphere/globe
    let position = data.position.as_ref().ok_or_else(|| 
        MyError::ValidationError("Position is missing.".to_string())
    )?;
    
    let ball = Ball::new(position);

    if !Globe::contains(&ball) {
        return Err(MyError::ValidationError("Ball is not on surface of sphere.".to_string()));
    }

    // Check the distance of the new ball from existing fixed balls
    if !is_valid_distance_from_others(position, &vec_position_alive_fixed_objects) {
        return Err(MyError::ValidationError("Ball is too close to other fixed objects.".to_string()));
    }

    // Check that UUID of new object is not among living objects.
    if map_alive_objects.contains_key(&data.object_uuid) {
        return Err(MyError::ValidationError("Object UUID is already in use.".to_string()));
    }

    // Validate color
    match &data.color {
        Some(color) => {
            if !validate_color(color) {
                return Err(MyError::ValidationError("Invalid color value provided. Please use one of the accepted 6-digit hex formats: #FF0000 (Red), #00FF00 (Green), #0000FF (Blue), or #FFFF00 (Yellow).".to_string()));
            }
        },
        None => {
            return Err(MyError::ValidationError("Color is required for insertion.".to_string()));
        }
    }

    // Validate impulse direction and magnitude if the ball is not fixed
    if !data.is_fixed {
        if let Some(impulse) = &data.impulse {
            validate_impulse_direction(position, impulse)?;
            validate_impulse_magnitude(impulse)?;
        } else {
            return Err(MyError::ValidationError("Impulse is required for dynamic objects.".to_string()));
        }
    }
    // Any additional validation checks can go here...

    Ok(())
}
