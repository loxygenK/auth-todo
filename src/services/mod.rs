use std::error::Error;

use serde::{Serialize, de::DeserializeOwned};

pub mod heart;
pub mod user;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Validation failed: {0}")]
    ValidationError(Box<dyn Error>),

    #[error(transparent)]
    ExternalError(Box<dyn Error>)
}

pub type ServiceResult<T> = Result<T, ServiceError>;

pub trait Service {
    type Response: Serialize;
    type Parameter: DeserializeOwned;

    fn serve(param: Self::Parameter) -> ServiceResult<Self::Response>;
}
