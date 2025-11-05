use crate::errors::{AppError, Result};
use crate::models::message::{CreateMessageRequest, MessageResponse};
use crate::models::room::{Entity as Room, RoomResponse};
use crate::services::jwt_service::Claims;
use crate::services::redis_service::CacheKey;
use crate::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::EntityTrait;

/// Get all rooms with Redis caching
pub async fn get_rooms(
    State(state): State<AppState>,
    _claims: Claims, // JWT middleware will inject this
) -> Result<Json<Vec<RoomResponse>>> {
    let cache_key = CacheKey::rooms_list();
    
    // Try to get from cache if Redis is enabled
    if let Some(redis) = &state.redis {
        match redis.get::<Vec<RoomResponse>>(&cache_key).await {
            Ok(Some(cached_rooms)) => {
                tracing::debug!("🎯 Returning {} rooms from cache", cached_rooms.len());
                return Ok(Json(cached_rooms));
            }
            Ok(None) => {
                tracing::debug!("❌ Cache miss, fetching from database");
            }
            Err(e) => {
                tracing::warn!("Redis error: {}. Falling back to database.", e);
            }
        }
    }
    
    // Fetch from database
    let rooms = Room::find().all(state.db.as_ref()).await?;
    let responses: Vec<RoomResponse> = rooms.into_iter().map(|r| r.into()).collect();
    
    // Cache the result if Redis is enabled
    if let Some(redis) = &state.redis {
        if let Err(e) = redis.set(&cache_key, &responses).await {
            tracing::warn!("Failed to cache rooms: {}", e);
        } else {
            tracing::debug!("💾 Cached {} rooms", responses.len());
        }
    }
    
    Ok(Json(responses))
}

pub async fn create_message(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    claims: Claims,
    Json(req): Json<CreateMessageRequest>,
) -> Result<Json<MessageResponse>> {
    let user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| AppError::InvalidToken)?;

    let message = state.message_service.create_message(user_id, room_id, req).await?;
    Ok(Json(message))
}
