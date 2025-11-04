use crate::errors::Result;
use crate::models::user::{LoginRequest, RegisterRequest};
use crate::services::auth_service::AuthResponse;
use crate::AppState;
use axum::{extract::State, Json};

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>> {
    let response = state.auth_service.register(req).await?;
    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let response = state.auth_service.login(req).await?;
    Ok(Json(response))
}
