use super::error_chain_fmt;
use actix_web::{http::StatusCode, ResponseError};

pub type HttpResult<T> = Result<T, HttpError>;

#[derive(thiserror::Error)]
pub enum HttpError {
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    UserError(&'static str),
    #[error("{0}")]
    NotFoundError(&'static str),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            HttpError::ValidationError(_) => StatusCode::BAD_REQUEST,
            HttpError::UserError(_) => StatusCode::BAD_REQUEST,
            HttpError::NotFoundError(_) => StatusCode::NOT_FOUND,
            HttpError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
