use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::MyError;
use crate::point_validation::*;
use redb::{Database, ReadableTable};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum Transaction {
    Insert(TransactionData),
    Delete(Uuid),
}

impl Transaction {
    pub fn validate_delete(&self, globe_id: &str, db: &Database) -> Result<(), MyError> {
        match self {
            Transaction::Delete(uuid_to_delete) => {
                let map_alive_objects = Self::get_alive_objects_map(globe_id, db)?;
                
                if !map_alive_objects.contains_key(uuid_to_delete) {
                    return Err(MyError::ValidationError("Cannot delete: UUID not found.".to_string()));
                }
                
                // Additional delete validations, if any, can be added here
                
                Ok(())
            },
            _ => Err(MyError::ValidationError("Invalid operation for validate_delete.".to_string())),
        }
    }

    fn get_alive_objects_map(globe_id: &str, db: &Database) -> Result<HashMap<Uuid, TransactionData>, MyError> {
        // ... Your code to build map_alive_objects ...

        Ok(map_alive_objects)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
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

impl Position {
    pub fn distance_squared(&self, other: &Position) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx*dx + dy*dy + dz*dz
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Velocity {
    v_x: f64,
    v_y: f64,
    v_z: f64,
}


fn parse_json(json_str: &str) -> Result<TransactionData, MyError> {
    serde_json::from_str(json_str).map_err(|err| MyError::JsonError(err.to_string()))
}

impl TransactionData {
    pub fn validate(&self, globe_id: &str, db: &Database) -> Result<(), MyError> {
        if self.is_fixed && self.velocity.is_some() {
            return Err(MyError::ValidationError("Velocity should be None for fixed objects.".to_string()));
        }

        //Check that incoming ball is not to close to other balls
        //Go through transactions for all fixed balls to find all living balls
        // read all transactions
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(crate::TABLE)?;
    
        let start = format!("{}--", globe_id);
        let end = format!("{}--{}", globe_id, "\u{10ffff}");
        let iter = table.range::<&str>(start.as_str()..end.as_str()).unwrap();

        let mut map_alive_objects: HashMap<Uuid, TransactionData> = HashMap::new();
        for item in iter {
            match item {
                Ok((_key, value)) => {
                    let data = parse_json(value.value()).unwrap();
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

        let mut vec_position_alive_fixed_objects: Vec<&Position> = Vec::new();
        for (key, value) in &map_alive_objects {
            if value.is_fixed{
                vec_position_alive_fixed_objects.push(&value.position);
            }
        }
        
        let sphere = Sphere {
            center: Position { x: 0.0, y: 0.0, z: 0.0 },
            radius: 1.0,
        };

        //Go through all alive_objects and check that all balls are minimum distance away from new ball
        //and that the new ball is on the surface of the sphere/globe
        if !sphere.contains(&self.position){
            return Err(MyError::ValidationError("Ball is not on surface of sphere.".to_string()))
        } 

        if !is_valid_distance_from_others(&self.position, &vec_position_alive_fixed_objects, 1.0 /* min_distance */){
            return Err(MyError::ValidationError("Ball is to close to other fixed balls.".to_string()))
        }
    
        //Check that uuid of new ball is not among living balls. I think its ok to reuse uuid if object is deleted?
        for key in map_alive_objects.keys() {
            if *key == self.object_uuid{
                return Err(MyError::ValidationError("Ball uuid is already in use.".to_string()))
            }
        }

        // You can add more validations here as needed
        Ok(())
    }
}