use auth_todo_derive::service;

use super::ServiceResult;

#[service("HeartBeatService", String)]
pub fn something(_: ()) -> ServiceResult<String> {
    Ok("I'm awake!".to_string())
}
