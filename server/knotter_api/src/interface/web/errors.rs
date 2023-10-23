use crate::domain::errors::my_error::MyError;

// Conversions specific to web or database interactions
impl From<redb::TransactionError> for MyError {
    // ...
}

impl ResponseError for MyError {
    // ...
}