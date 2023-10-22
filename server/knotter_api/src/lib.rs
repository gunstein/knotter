mod errors;

use redb::Database;
use std::{sync::Arc, fs};
use errors::MyError;

//Don't really need a lib.rs right now, but it might be if I want to use #[actix_web::test] to test only parts of end points.
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