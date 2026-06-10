use certify_service_backend::{
    Application, app_state::AppState, get_postgres_pool, services::CertificateStore,
    utils::constants::DATABASE_URL,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;

    let certificate_store = Arc::new(RwLock::new(CertificateStore::new(pg_pool)));
    let app_state = AppState::new(certificate_store);
    let app = Application::build(app_state, "0.0.0.0:9168")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run service")
}

async fn configure_postgresql() -> PgPool {
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}
