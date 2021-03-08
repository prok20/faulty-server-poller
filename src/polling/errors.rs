use actix_web::{HttpResponse, ResponseError};
use async_channel::TrySendError;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Too many requests")]
    TooManyRequests,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal server error, please try again later"),
            ServiceError::TooManyRequests => {
                HttpResponse::TooManyRequests().json("Too many requests, please try again later")
            }
        }
    }
}

impl<T> From<TrySendError<T>> for ServiceError {
    fn from(e: TrySendError<T>) -> Self {
        match e {
            TrySendError::Closed(_) => Self::InternalServerError,
            TrySendError::Full(_) => Self::TooManyRequests,
        }
    }
}

pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
