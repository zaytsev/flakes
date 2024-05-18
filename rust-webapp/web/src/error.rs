use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::view::{common::page_template, error::error_message_fragment};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database connection pool error")]
    PoolError(#[from] db::ConnectionPoolError),

    #[error("Database error")]
    DatabaseError(#[from] db::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::PoolError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Error::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
        (
            status,
            page_template("Error", error_message_fragment(&message, None)),
        )
            .into_response()
    }
}
