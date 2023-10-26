use redb::Database;
use std::{sync::Arc, fs, collections::HashMap};
use chrono::{self, Utc};
use redb::{ReadableTable, TableDefinition};
use crate::domain::errors::my_error::MyError;
use uuid::Uuid;
use crate::domain::models::ball_entity::BallEntity;

pub const TABLE: TableDefinition<&str, &str> = TableDefinition::new("knotter_log");

pub struct KeyValueStore {
    db: Arc<Database>,
}

pub trait KeyValueStoreTrait {
    fn get_alive_objects_map(&self, globe_id: &str) -> Result<HashMap<Uuid, BallEntity>, MyError>;
    // Add other methods here as needed...
}

impl KeyValueStoreTrait for KeyValueStore {
    fn get_alive_objects_map(&self, globe_id: &str) -> Result<HashMap<Uuid, BallEntity>, MyError> {
        // Read all transactions
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;
    
        let start = format!("{}--", globe_id);
        let end = format!("{}--{}", globe_id, "\u{10ffff}");
        let iter = table.range::<&str>(start.as_str()..end.as_str()).unwrap();
    
        let mut map_alive_objects: HashMap<Uuid, BallEntity> = HashMap::new();
        for item in iter {
            match item {
                Ok((_key, value)) => {
                    let data = Self::parse_json(value.value())?;
                    if data.is_insert && data.is_fixed {
                        map_alive_objects.insert(data.uuid, data);
                    } else {
                        map_alive_objects.remove(&data.uuid);
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

impl KeyValueStore {
    pub fn new(db: Arc<Database>) -> Self {
        KeyValueStore { db }
    }

    pub fn insert(&self, globe_id: &str, serialized_data: &str, timestamp: &str) -> Result<(), MyError> {
        let key = self.construct_key(globe_id, timestamp);
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            table.insert(&*key, serialized_data)?;
        }
        write_txn.commit()?;
        Ok(())
    }
    
    // This function now only constructs the key given a timestamp
    fn construct_key(&self, globe_id: &str, timestamp: &str) -> String {
        format!("{}--{}", globe_id, timestamp)
    }

    pub fn setup_database(test_db: bool) -> Result<Arc<Database>, MyError> {
        let db_filename = if test_db {
            "test_knotter_db.redb"
        } else {
            "knotter_db.redb"
        };
    
        if test_db {
            // Try to delete the test database file
            let _ = fs::remove_file(db_filename);
        }
    
        let db = Database::create(db_filename)
            .map_err(|e| MyError::DatabaseError(e.to_string()))?;
    
        Ok(Arc::new(db))
    }

    fn generate_key(globe_id: &str) -> (String, String) {
        let now = Utc::now();
        let nanoseconds_since_epoch = (now.timestamp_subsec_nanos() as i64 + now.timestamp() * 1_000_000_000).to_string();
        let key = format!("{}--{}", globe_id, nanoseconds_since_epoch);

        (key, nanoseconds_since_epoch)
    }
 
    fn parse_json(json_str: &str) -> Result<BallEntity, MyError> {
        serde_json::from_str(json_str).map_err(|err| MyError::JsonError(err.to_string()))
    }
}
