use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{app_state::AppState, domain::AppError};

pub async fn fetch_all_certificates(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let certificate_store = state.certificate_store.read().await;

    let certificates = certificate_store
        .get_all_certificates()
        .await
        .map_err(AppError::Database)?;

    Ok((StatusCode::OK, Json(certificates)))
}
