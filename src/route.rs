use std::sync::Arc;

use axum::{Router, middleware, routing::get};
use sea_orm::DatabaseConnection;

use crate::middleware::print_request_response;

pub mod admin;

struct AppState {
    connection_pool: DatabaseConnection,
}

pub fn get_router(pool: DatabaseConnection) -> Router {
    let shared_state = Arc::new(AppState {
        connection_pool: pool,
    });
    Router::new()
        .route("/ping", get(|| async { "pong" }))
        .layer(middleware::from_fn(print_request_response))
        .with_state(shared_state)
}
