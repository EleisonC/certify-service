use std::sync::Arc;
use tokio::sync::RwLock;

use crate::services::CertificateStore;

pub type CertificateStoreType = Arc<RwLock<CertificateStore>>;

#[derive(Clone)]
pub struct AppState {
    pub certificate_store: CertificateStoreType,
}

impl AppState {
    pub fn new(certificate_store: CertificateStoreType) -> Self {
        Self { certificate_store }
    }
}
