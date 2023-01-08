use std::error::Error;

use serde::Serialize;

use super::ApiError;

#[derive(Serialize)]
#[serde(tag = "status")]
pub enum Response<T: Serialize> {
    Success { payload: T },
    Failure { error: ApiError },
}
