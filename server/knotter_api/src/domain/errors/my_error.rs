use redb;
use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    NotFound,
    DatabaseError(String),
    ValidationError(String),
    InternalServerError(String),
    JsonError(String),
    // ... other errors
}

impl From<redb::TransactionError> for MyError {
    fn from(err: redb::TransactionError) -> Self {
        MyError::DatabaseError(err.to_string())
    }
}

impl From<redb::TableError> for MyError {
    fn from(err: redb::TableError) -> Self {
        MyError::DatabaseError(err.to_string())
    }
}

impl From<redb::StorageError> for MyError {
    fn from(err: redb::StorageError) -> Self {
        MyError::DatabaseError(err.to_string())
    }
}

impl From<redb::CommitError> for MyError {
    fn from(err: redb::CommitError) -> Self {
        MyError::DatabaseError(err.to_string())
    }
}


impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::NotFound => write!(f, "Not found"),
            MyError::DatabaseError(ref message) => write!(f, "Database error: {}", message),
            MyError::ValidationError(ref message) => write!(f, "Validation error: {}", message),
            MyError::InternalServerError(ref message) => write!(f, "Internal error: {}", message),
            MyError::JsonError(ref message) => write!(f, "JSON serialization/deserialization error: {}", message),
            // ... other error variants
        }
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::DatabaseError(ref message) => HttpResponse::InternalServerError().json(message),
            MyError::ValidationError(ref message) => HttpResponse::BadRequest().json(message),
            MyError::InternalServerError(ref message) => HttpResponse::InternalServerError().json(message),
            MyError::JsonError(ref message) => HttpResponse::BadRequest().json(message), // You can choose an appropriate status for JSON errors
            // ... other error mappings
        }
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> Self {
        MyError::JsonError(err.to_string())
    }
}