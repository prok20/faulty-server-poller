use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
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

pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
