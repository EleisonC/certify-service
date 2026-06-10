use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{app_state::AppState, domain::AppError};

pub async fn fetch_certificate(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let certificate_store = state.certificate_store.read().await;

    let result = certificate_store
        .get_certificate_by_id(id)
        .await
        .map_err(|e| AppError::Database(e))?
        .ok_or(AppError::NotFound)?;

    Ok((StatusCode::OK, Json(result)))
}
