use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error")]
    SQLx(#[from] sqlx::Error),
    #[error("internal server error")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::SQLx(error) => {
                error!(?error, "SQLx error");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::Other(error) => {
                error!(?error, "Other error");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
