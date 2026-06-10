use app_state::AppState;
use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
    routing::{get, post},
};
use axum_server::tls_rustls::RustlsConfig;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{error::Error, net::TcpListener};
use tower_http::cors::CorsLayer;

use crate::routes::{create_certificate, fetch_all_certificates, fetch_certificate, health_check};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

// This struct encapsulates our application-related logic.
pub struct Application {
    // address is exposed as a public field
    // so we have access to it in tests.
    router: Router,
    listener: TcpListener, // std listener, so we can read the bound address
    tls_config: RustlsConfig,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let cors = CorsLayer::new()
            .allow_origin(["http://localhost:3000".parse()?])
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([CONTENT_TYPE]);
        let router = Router::new()
            .route("/health-check", get(health_check))
            .route("/certificate", post(create_certificate))
            .route("/certificate/{id}", get(fetch_certificate))
            .route("/certificates", get(fetch_all_certificates))
            .layer(cors)
            .with_state(app_state);

        let tls_config = RustlsConfig::from_pem_file("./certs/cert.pem", "./certs/key.pem").await?;

        let listener = TcpListener::bind(address)?;
        let address = listener.local_addr()?.to_string();
        listener
            .set_nonblocking(true)
            .expect("Failed to set non-blocking mode");

        Ok(Application {
            router,
            listener,
            tls_config,
            address,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        axum_server::from_tcp_rustls(self.listener, self.tls_config)
            .expect("Failed to create TLS server")
            .serve(self.router.into_make_service())
            .await
    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(url).await
}
