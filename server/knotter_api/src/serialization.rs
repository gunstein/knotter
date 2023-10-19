use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::MyError;
use crate::point_validation::*;
use redb::{Database, ReadableTable};
use std::collections::HashMap;
use nalgebra::Vector3;

use crate::impulse_validation::{validate_impulse_direction, validate_impulse_magnitude};


#[derive(Serialize, Deserialize, Debug)]
pub enum Transaction {
    Insert(TransactionData),
}

impl Transaction {
    pub fn validate_delete(uuid_to_delete: &Uuid, globe_id: &str, db: &Database) -> Result<(), MyError> {
        let map_alive_objects = Self::get_alive_objects_map(globe_id, db)?;
        
        if !map_alive_objects.contains_key(uuid_to_delete) {
            return Err(MyError::ValidationError("Cannot delete: UUID not found.".to_string()));
        }
        
        // Additional delete validations, if any, can be added here
        
        Ok(())
    }
    
    fn get_alive_objects_map(globe_id: &str, db: &Database) -> Result<HashMap<Uuid, TransactionData>, MyError> {
        // Read all transactions
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(crate::TABLE)?;

        let start = format!("{}--", globe_id);
        let end = format!("{}--{}", globe_id, "\u{10ffff}");
        let iter = table.range::<&str>(start.as_str()..end.as_str()).unwrap();

        let mut map_alive_objects: HashMap<Uuid, TransactionData> = HashMap::new();
        for item in iter {
            match item {
                Ok((_key, value)) => {
                    let data = parse_json(value.value())?;
                    if data.is_insert && data.is_fixed {
                        map_alive_objects.insert(data.object_uuid, data);
                    } else {
                        map_alive_objects.remove(&data.object_uuid);
                    }
                }
                Err(err) => {
                    return Err(MyError::DatabaseError(format!("Fetching of data failed: {}", err)))
                }
            }
        }

        Ok(map_alive_objects)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TransactionData {
    is_fixed: bool,
    is_insert: bool,
    object_uuid: Uuid,
    color: Option<String>, 
    position: Option<Position>,
    impulse: Option<Impulse>,
}

impl TransactionData {
    pub fn new(uuid: Uuid, is_insert: bool) -> Self {
        TransactionData {
            is_fixed: false,
            is_insert,
            object_uuid: uuid,
            color: None,
            position: None,
            impulse: None,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn distance_squared(&self, other: &Position) -> f64 {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Impulse {
    v_x: f64,
    v_y: f64,
    v_z: f64,
}

impl Impulse {
    // Conversion to nalgebra::Vector3<f64>
    pub fn to_vector3(&self) -> Vector3<f64> {
        Vector3::new(self.v_x, self.v_y, self.v_z)
    }
}

fn parse_json(json_str: &str) -> Result<TransactionData, MyError> {
    serde_json::from_str(json_str).map_err(|err| MyError::JsonError(err.to_string()))
}

impl TransactionData {
    pub fn validate_insert(&self, globe_id: &str, db: &Database) -> Result<(), MyError> {
        // Preliminary checks
        if self.is_fixed && self.impulse.is_some() {
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
        let position = self.position.as_ref().ok_or_else(|| 
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
        if map_alive_objects.contains_key(&self.object_uuid) {
            return Err(MyError::ValidationError("Object UUID is already in use.".to_string()));
        }

        //Validate color
        match &self.color {
            Some(color) => {
                if !crate::validate_color(color) {
                    return Err(MyError::ValidationError("Invalid color value provided. Please use one of the accepted 6-digit hex formats: #FF0000 (Red), #00FF00 (Green), #0000FF (Blue), or #FFFF00 (Yellow).".to_string()));
                }
            },
            None => {
                return Err(MyError::ValidationError("Color is required for insertion.".to_string()));
            }
        }

        // Validate impulse direction and magnitude if the ball is not fixed
        if !self.is_fixed {
            if let Some(impulse) = &self.impulse {
                validate_impulse_direction(position, impulse)?;
                validate_impulse_magnitude(impulse)?;
            } else {
                return Err(MyError::ValidationError("Impulse is required for dynamic objects.".to_string()));
            }
        }
        // Any additional validation checks can go here...

        Ok(())
    }

    
}