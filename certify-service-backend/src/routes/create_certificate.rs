use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AppError, NewCertificate},
};

pub async fn create_certificate(
    State(state): State<AppState>,
    Json(payload): Json<CertificateInput>,
) -> Result<impl IntoResponse, AppError> {
    let data = NewCertificate::parse_pem(&payload.pem_data)?;

    let mut certificate_store = state.certificate_store.write().await;

    // if certificate_store
    //     .get_certificate_by_serial_number(&data.serial_number.as_deref().expect("msg"))
    //     .await
    //     .is_ok()
    // {
    //     return Err(AppError::Validation(
    //         "Certificate with this serial number already exists".to_string(),
    //     ));
    // }

    let result = certificate_store
        .insert_certificate(&data)
        .await
        .map_err(|e| AppError::Database(e))?;

    Ok((StatusCode::CREATED, Json(result)))
}

#[derive(Debug, Clone, Deserialize)]
pub struct CertificateInput {
    pub pem_data: String,
}
