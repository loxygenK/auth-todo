use std::sync::Arc;

use axum::routing::get;

use crate::{services::heart::HeartBeatService, api::{ctx::Context, scheme::Response, use_service, serialize}};

pub fn heart_route(ctx: Arc<Context>) -> axum::Router {
    axum::Router::new()
        .route("/health", get(heartbeat))
        .with_state(ctx)
}

async fn heartbeat() -> axum::Json<Response<String>> {
    serialize(use_service::<HeartBeatService>(()).await)
}

