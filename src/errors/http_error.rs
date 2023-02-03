use super::error_chain_fmt;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

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

impl HttpError {
    pub fn name(&self) -> String {
        match self {
            Self::ValidationError(_) => "ValidationError".into(),
            Self::UserError(_) => "UserError".into(),
            Self::NotFoundError(_) => "NotFoundError".into(),
            Self::UnexpectedError(_) => "UnexpectedError".into(),
        }
    }
}

impl std::fmt::Debug for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::UserError(_) => StatusCode::BAD_REQUEST,
            Self::NotFoundError(_) => StatusCode::NOT_FOUND,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponseBody::from(self))
    }
}

#[derive(Serialize)]
pub struct ErrorResponseBody {
    pub name: String,
    pub message: String,
}

impl From<&HttpError> for ErrorResponseBody {
    fn from(err: &HttpError) -> Self {
        Self {
            name: err.name(),
            message: err.to_string(),
        }
    }
}
