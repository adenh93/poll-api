use super::error_chain_fmt;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

pub type HttpResult<T> = Result<T, HttpError>;

#[derive(thiserror::Error)]
pub enum HttpError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
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
        let status_code = self.status_code();

        match self {
            Self::ValidationError(err) => {
                HttpResponse::build(status_code).json(ValidationErrorResponse::from(err))
            }
            _ => HttpResponse::build(status_code).json(GenericErrorResponse::from(self)),
        }
    }
}

impl From<&validator::ValidationErrors> for ValidationErrorResponse {
    fn from(err: &validator::ValidationErrors) -> Self {
        let field_errors = err
            .field_errors()
            .iter()
            .map(|(key, value)| ValidationFieldError {
                field: key.to_string(),
                errors: value.iter().map(|e| e.code.to_string()).collect(),
            })
            .collect();

        Self {
            name: "ValidationError".into(),
            message: err.to_string(),
            field_errors,
        }
    }
}

#[derive(Serialize)]
pub struct GenericErrorResponse {
    pub name: String,
    pub message: String,
}

impl From<&HttpError> for GenericErrorResponse {
    fn from(err: &HttpError) -> Self {
        Self {
            name: err.name(),
            message: err.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ValidationErrorResponse {
    pub name: String,
    pub message: String,
    pub field_errors: Vec<ValidationFieldError>,
}

#[derive(Serialize)]
pub struct ValidationFieldError {
    pub field: String,
    pub errors: Vec<String>,
}
