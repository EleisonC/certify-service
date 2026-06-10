use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

lazy_static! {
    pub static ref DATABASE_URL: String = set_db_url();
}

fn set_db_url() -> String {
    dotenv().ok();
    std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set.")
}

pub mod env {
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
}
