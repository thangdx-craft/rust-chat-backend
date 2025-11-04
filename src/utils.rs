use crate::errors::AppError;
use crate::services::jwt_service::{Claims, JwtService};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

// Implement FromRequestParts for Claims to extract JWT from Authorization header
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
    Arc<JwtService>: FromRef<S>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract JWT service from state
        let jwt_service: Arc<JwtService> = Arc::from_ref(state);

        // Get authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::AuthError("Missing authorization header".to_string()).into_response())?;

        // Extract token from "Bearer <token>"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::AuthError("Invalid authorization format".to_string()).into_response())?;

        // Verify token
        jwt_service
            .verify_token(token)
            .map_err(|e| e.into_response())
    }
}
