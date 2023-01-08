use std::net::SocketAddr;
use std::sync::Arc;

use serde::Serialize;

use crate::services::{ServiceError, Service};

use self::{ctx::Context, scheme::Response};

pub mod ctx;
pub mod routes;
pub mod scheme;
pub mod se;

#[derive(thiserror::Error, Debug, Serialize)]
pub enum ApiError {
    #[error("Validation failed (malformed request?): {0}")]
    #[serde(serialize_with = "self::se::with_display")]
    ValidateError(String),

    #[error(transparent)]
    #[serde(serialize_with = "self::se::with_display")]
    ServiceError(#[from] ServiceError)
}

pub type ApiResult<T> = Result<T, ApiError>;

pub(self) fn serialize<T: Serialize>(result: ApiResult<T>) -> axum::Json<Response<T>> {
    axum::Json(match result {
        Ok(payload) => Response::Success { payload },
        Err(error) => Response::Failure { error }
    })
}

pub(self) async fn use_service<T: Service>(param: T::Parameter) -> ApiResult<T::Response> {
    T::serve(param).map_err(Into::into)
}

pub async fn start_api(ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Arc::new(ctx);
    let app = routes::route(ctx);

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
        .map_err(Box::<dyn std::error::Error>::from)
}
