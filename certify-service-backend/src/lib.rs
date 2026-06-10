use app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
    serve::Serve,
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::error::Error;
use tokio::net::TcpListener;

use crate::routes::{create_certificate, fetch_certificate, health_check};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .route("/health-check", get(health_check))
            .route("/certificate", post(create_certificate))
            .route("/certificate/{id}", get(fetch_certificate))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(url).await
}
