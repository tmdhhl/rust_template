use std::sync::{Arc, Mutex};

use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

use crate::{configuration::ApplicationSettings, route::translate::translate_link};

mod order_detail;
mod translate;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Mutex<AppStateInner>>,
}

#[derive(Clone)]
struct AppStateInner {
    #[allow(dead_code)]
    connection_pool: DatabaseConnection,
    app_settings: ApplicationSettings,
}

impl AppState {
    pub fn new(pool: DatabaseConnection, app_settings: ApplicationSettings) -> Self {
        let inner = AppStateInner {
            connection_pool: pool,
            app_settings,
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/translate_link", get(translate_link))
        .route("/order_detail", get(translate_link))
        .with_state(state)
}
