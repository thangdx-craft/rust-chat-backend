use crate::errors::{AppError, Result};
use crate::models::message::{CreateMessageRequest, MessageResponse};
use crate::models::room::{Entity as Room, RoomResponse};
use crate::services::jwt_service::Claims;
use crate::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::EntityTrait;

pub async fn get_rooms(
    State(state): State<AppState>,
    _claims: Claims, // JWT middleware will inject this
) -> Result<Json<Vec<RoomResponse>>> {
    let rooms = Room::find().all(state.db.as_ref()).await?;
    let responses: Vec<RoomResponse> = rooms.into_iter().map(|r| r.into()).collect();
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
