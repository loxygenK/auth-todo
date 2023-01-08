use std::sync::Arc;

use axum::Router;

use self::heart::heart_route;

use super::ctx::Context;

mod heart;

pub fn route(ctx: Arc<Context>) -> Router {
    Router::new()
        .nest("/", heart_route(ctx))
}
