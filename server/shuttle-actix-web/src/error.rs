use actix_web::{error, error::Error as ActixError};
use engine::types::Error as EngineError;

pub(crate) trait IntoActixError {
    fn into_actix_error(self) -> ActixError;
}

impl IntoActixError for EngineError {
    fn into_actix_error(self) -> ActixError {
        match self {
            EngineError::Validation(e) => error::ErrorBadRequest(e),
            EngineError::RowNotFound => error::ErrorNotFound("Record not found"),
            EngineError::Conflict(e) => error::ErrorConflict(e),
            _ => error::ErrorInternalServerError("Internal server error"),
        }
    }
}