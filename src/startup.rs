use std::io::Error;

use axum::{Router, serve::Serve};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::net::TcpListener;

use crate::{
    configuration,
    route::{AppState, get_router},
};

pub struct Application {
    port: u16,
    server: Serve<TcpListener, Router, Router>,
}

impl Application {
    pub async fn build(config: configuration::Settings) -> anyhow::Result<Application> {
        let connection_pool = get_connection_pool(config.db.build()).await?;
        let app_state = AppState::new(connection_pool, config.application.clone());
        let router = get_router(app_state);

        let port = config.application.port;
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

        let server = axum::serve(listener, router);

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> std::result::Result<(), Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

async fn get_connection_pool(opts: ConnectOptions) -> anyhow::Result<DatabaseConnection> {
    Database::connect(opts)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
