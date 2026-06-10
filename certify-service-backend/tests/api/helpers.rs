use certify_service_backend::{
    Application, app_state::AppState, get_postgres_pool, services::CertificateStore,
    utils::constants::DATABASE_URL,
};
use sqlx::{
    Connection, Executor, PgConnection, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub db_name: String,
    pub clean_up_called: bool,
}

impl TestApp {
    pub async fn new() -> Self {
        let db_name = Uuid::new_v4().to_string();
        let pg_pool = configure_postgresql(&db_name).await;

        let certificate_store = Arc::new(RwLock::new(CertificateStore::new(pg_pool)));
        let app_state = AppState::new(certificate_store);
        let app = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        Self {
            address,
            http_client,
            db_name,
            clean_up_called: false,
        }
    }

    pub async fn get_health_check(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/health-check", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn create_new_certificate<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/certificate", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn fetch_certificate_by_id(&self, id: &str) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/certificate/{}", &self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn clean_up(&mut self) {
        if self.clean_up_called {
            return;
        }

        delete_database(&self.db_name).await;

        self.clean_up_called = true;
    }

    // pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    // where
    //     Body: serde::Serialize,
    // {
    //     self.http_client
    //         .post(&format!("{}/signup", &self.address))
    //         .json(body)
    //         .send()
    //         .await
    //         .expect("Failed to execute request.")
    // }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.clean_up_called {
            panic!("You must clean up db: {}", self.db_name)
        } else {
            println!("Dropping Test DB: {}", self.db_name);
        }
    }
}

async fn delete_database(db_name: &str) {
    let postgresql_conn_url = DATABASE_URL.to_owned();

    let connection_options = PgConnectOptions::from_str(&postgresql_conn_url)
        .expect("Failed to parse PostgreSQL connection string");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");

    // Kill active connections to the database
    connection
        .execute(
            format!(
                r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                  AND pid <> pg_backend_pid();
        "#,
                db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to drop the database.");

    // Drop the database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to drop the database.");
}

async fn configure_postgresql(db_name: &str) -> PgPool {
    let postgresql_conn_url = "postgres://pos:[your-password]@localhost:5432".to_owned();

    configure_database(&postgresql_conn_url, db_name).await;

    let postgresql_conn_url_with_db = format!("{}/{}", postgresql_conn_url, db_name);

    get_postgres_pool(&postgresql_conn_url_with_db)
        .await
        .expect("Failed to create Postgres connection pool!")
}

async fn configure_database(db_conn_string: &str, db_name: &str) {
    let connection = PgPoolOptions::new()
        .connect(db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Create database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database.");

    let db_conn_string = format!("{}/{}", db_conn_string, db_name);

    let connection = PgPoolOptions::new()
        .connect(&db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Migrate database
    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to migrate the database");
}
