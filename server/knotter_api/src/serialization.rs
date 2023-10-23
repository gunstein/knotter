use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::MyError;
use crate::validation::point_validation::*;
use redb::{Database, ReadableTable};
use std::collections::HashMap;
use nalgebra::Vector3;
use crate::validation::impulse_validation::{validate_impulse_direction, validate_impulse_magnitude};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Transaction {
    Insert(TransactionData),
}

impl Transaction {

    
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


fn parse_json(json_str: &str) -> Result<TransactionData, MyError> {
    serde_json::from_str(json_str).map_err(|err| MyError::JsonError(err.to_string()))
}



 
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_insert_serialization() {
        let uuid = Uuid::new_v4(); // Generates a new random UUID

        let transaction_data = TransactionData {
            is_insert: true,
            is_fixed: true,
            object_uuid: uuid,
            color: Some("red".to_string()),
            position: Some(Position { x: 1.0, y: 2.0, z: 3.0 }),
            impulse: Some(Impulse { v_x: 1.0, v_y: 2.0, v_z: 3.0 }),
        };
        
        let insert = Transaction::Insert(transaction_data);

        // Step 3: Serialize
        let serialized = serde_json::to_string(&insert).expect("Failed to serialize");

        // Step 4: Assert Serialized Output
        //assert_eq!(serialized, format!("{{\"insert\":{{\"is_fixed\":true,\"is_insert\":true,\"object_uuid\":\"{}\",\"color\":\"red\",\"position\":{{\"x\":1.0,\"y\":2.0,\"z\":3.0}},\"impulse\":{{\"v_x\":1.0,\"v_y\":2.0,\"v_z\":3.0}}}}}}", uuid.to_string()));
        assert_eq!(serialized, format!(r#"{{"insert":{{"is_fixed":true,"is_insert":true,"object_uuid":"{}","color":"red","position":{{"x":1.0,"y":2.0,"z":3.0}},"impulse":{{"v_x":1.0,"v_y":2.0,"v_z":3.0}}}}}}"#, uuid.to_string()));

        // Step 5: Deserialize
        let deserialized: Transaction = serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Step 6: Assert Deserialized Data
        assert_eq!(deserialized, insert);
    }

    #[test]
    fn test_validate_insert() {
        let uuid = Uuid::new_v4(); // Generates a new random UUID

        let transaction_data = TransactionData {
            is_insert: true,
            is_fixed: true,
            object_uuid: uuid,
            color: Some("red".to_string()),
            position: Some(Position { x: 1.0, y: 2.0, z: 3.0 }),
            impulse: Some(Impulse { v_x: 1.0, v_y: 2.0, v_z: 3.0 }),
        };
        
        let insert = Transaction::Insert(transaction_data);

        // Step 3: Serialize
        let serialized = serde_json::to_string(&insert).expect("Failed to serialize");

        // Step 4: Assert Serialized Output
        //assert_eq!(serialized, format!("{{\"insert\":{{\"is_fixed\":true,\"is_insert\":true,\"object_uuid\":\"{}\",\"color\":\"red\",\"position\":{{\"x\":1.0,\"y\":2.0,\"z\":3.0}},\"impulse\":{{\"v_x\":1.0,\"v_y\":2.0,\"v_z\":3.0}}}}}}", uuid.to_string()));
        assert_eq!(serialized, format!(r#"{{"insert":{{"is_fixed":true,"is_insert":true,"object_uuid":"{}","color":"red","position":{{"x":1.0,"y":2.0,"z":3.0}},"impulse":{{"v_x":1.0,"v_y":2.0,"v_z":3.0}}}}}}"#, uuid.to_string()));

        // Step 5: Deserialize
        let deserialized: Transaction = serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Step 6: Assert Deserialized Data
        assert_eq!(deserialized, insert);
    }    
}