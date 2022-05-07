use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum CustomError {
    Database(String),
}

// So that errors get printed to the browser?
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };

        format!("status = {}, message = {}", status, error_message).into_response()
    }
}

impl From<tokio_postgres::Error> for CustomError {
    fn from(err: tokio_postgres::Error) -> CustomError {
        CustomError::Database(err.to_string())
    }
}

impl From<deadpool_postgres::PoolError> for CustomError {
    fn from(err: deadpool_postgres::PoolError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}
