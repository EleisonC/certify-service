use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

lazy_static! {
    pub static ref DATABASE_URL: String = set_db_url();
    pub static ref POSTGRES_PASSWORD: String = set_postgres_password();
}

fn set_db_url() -> String {
    dotenv().ok();
    std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set.")
}

fn set_postgres_password() -> String {
    dotenv().ok();
    std_env::var(env::POSTGRES_PASSWORD_ENV_VAR).expect("POSTGRES_PASSWORD must be set.")
}

pub mod env {
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const POSTGRES_PASSWORD_ENV_VAR: &str = "POSTGRES_PASSWORD";
}
