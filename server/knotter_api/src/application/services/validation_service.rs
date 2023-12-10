use crate::domain::errors::my_error::MyError;
use log::debug;
use regex::Regex;
use uuid::Uuid;
use crate::infrastructure::database::key_value_store::KeyValueStoreTrait;
use crate::domain::models::ball_entity::{BallEntity, PositionEntity};
use crate::application::services::validation::ball_position_validator::*;
use crate::application::services::validation::ball_impulse_validator::*;


pub struct ValidationService {
    // ... any shared state or configuration
}

impl ValidationService {
    pub fn new(/* any dependencies */) -> Self {
        Self {
            // ... initialization
        }
    }
    
    pub fn validate_delete<T: KeyValueStoreTrait>(uuid_to_delete: &Uuid, globe_id: &str, key_value_store: &T) -> Result<(), MyError> {
        let map_alive_objects = key_value_store.get_alive_objects_map(globe_id)?;
        
        if !map_alive_objects.contains_key(uuid_to_delete) {
            return Err(MyError::ValidationError("Cannot delete: UUID not found.".to_string()));
        }
        
        // Additional delete validations, if any, can be added here
        
        Ok(())
    }

    pub fn validate_insert<T: KeyValueStoreTrait>(&self, ball_entity: &BallEntity, globe_id: &str, key_value_store: &T) -> Result<(), MyError> {
        // Preliminary checks
        if ball_entity.is_fixed && ball_entity.impulse.is_some() {
            return Err(MyError::ValidationError("Velocity should be None for fixed objects.".to_string()));
        }
        //debug!("insert_ball_dto {:?}", insert_ball_dto);
        debug!("validate 1" );
        // Retrieve all alive objects
        let map_alive_objects = key_value_store.get_alive_objects_map(globe_id)?;
        debug!("validate 2" );
        let mut vec_position_alive_fixed_objects: Vec<&PositionEntity> = Vec::new();
        for value in map_alive_objects.values() {
            if value.is_fixed {
                if let Some(position) = &value.position {
                    vec_position_alive_fixed_objects.push(position);
                }
            }
        }
        debug!("validate 3" );
        // Check that the new object is on the surface of the sphere/globe
        let position = ball_entity.position.as_ref().ok_or_else(|| 
            MyError::ValidationError("Position is missing.".to_string())
        )?;
        debug!("validate 4" );
        if !Globe::contains(&position) {
            return Err(MyError::ValidationError("Ball is not on surface of sphere.".to_string()));
        }
        debug!("validate 5" );
        // Check the distance of the new ball from existing fixed balls
        if !is_valid_distance_from_others(position, &vec_position_alive_fixed_objects) {
            return Err(MyError::ValidationError("Ball is too close to other fixed objects.".to_string()));
        }
        debug!("validate 6" );
        // Check that UUID of new object is not among living objects.
        if map_alive_objects.contains_key(&ball_entity.uuid) {
            return Err(MyError::ValidationError("Object UUID is already in use.".to_string()));
        }
        debug!("validate 7" );
        // Validate color
        match &ball_entity.color {
            Some(color) => {
                if !ValidationService::validate_color(color) {
                    return Err(MyError::ValidationError(format!("Invalid color value provided: {}", color)));
                }
            },
            None => {
                return Err(MyError::ValidationError("Color is required for insertion.".to_string()));
            }
        }
        debug!("validate 8" );
        // Validate impulse direction and magnitude if the ball is not fixed
        if !ball_entity.is_fixed {
            if let Some(impulse) = &ball_entity.impulse {
                validate_impulse_direction(position, impulse)?;
                validate_impulse_magnitude(impulse)?;
            } else {
                return Err(MyError::ValidationError("Impulse is required for dynamic objects.".to_string()));
            }
        }
        // Any additional validation checks can go here...
    
        Ok(())
    }

    fn validate_color(color: &str) -> bool {
        let re = Regex::new(r"^#([A-Fa-f0-9]{8})$").unwrap();
        re.is_match(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::ball_entity::{BallEntity, PositionEntity};
    use std::collections::HashMap;

    // Mock implementation of KeyValueStore to be used in tests
    struct MockKeyValueStore;

    impl MockKeyValueStore {
        fn get_alive_objects_map(&self, _globe_id: &str) -> Result<HashMap<Uuid, BallEntity>, MyError> {
            Ok(HashMap::new())  // Returning an empty map for simplicity. Adjust this as needed.
        }
    }

    impl KeyValueStoreTrait for MockKeyValueStore {
        fn get_alive_objects_map(&self, _globe_id: &str) -> Result<HashMap<Uuid, BallEntity>, MyError> {
            Ok(HashMap::new())  // Mock implementation
        }
        // Mock other methods here as needed...
    }

    #[test]
    fn test_validate_insert_ball_not_on_surface() {
        let validation_service = ValidationService::new();
        let key_value_store = MockKeyValueStore;

        let ball_entity = BallEntity {
            is_insert: true,
            uuid: Uuid::new_v4(),
            position: Some(PositionEntity { x: 10.0, y: 10.0, z: 10.0 }),
            color: Some("#ff0000".to_string()),
            is_fixed: true,
            impulse: None,
            // Add any other required fields here
        };

        let result = validation_service.validate_insert(&ball_entity, "some_globe_id", &key_value_store);

        match result {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(e) => {
                match e {
                    MyError::ValidationError(msg) => {
                        assert_eq!(msg, "Ball is not on surface of sphere.");
                    },
                    _ => panic!("Expected ValidationError but got a different error"),
                }
            }
        }
    }
}
