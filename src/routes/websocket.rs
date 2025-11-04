use crate::errors::AppError;
use crate::models::message::CreateMessageRequest;
use crate::services::jwt_service::Claims;
use crate::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

// WebSocket message types
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "message")]
    Message { content: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsBroadcast {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub sender: String,
    pub sender_id: i32,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct WsQuery {
    token: String,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<i32>,
    Query(query): Query<WsQuery>,
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    // Verify JWT token
    let claims = state.jwt_service.verify_token(&query.token)?;

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, room_id, claims, state)))
}

async fn handle_socket(socket: WebSocket, room_id: i32, claims: Claims, state: AppState) {
    let (sender, receiver) = socket.split();

    // Get or create room channel
    let tx = {
        let mut rooms = state.rooms.write().await;
        rooms
            .entry(room_id)
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    };

    let rx = tx.subscribe();

    let user_id = claims.sub.parse::<i32>().unwrap_or(0);
    let username = claims.email.clone();

    // Spawn task to send messages to this client
    let mut send_task = tokio::spawn(send_messages(sender, rx));

    // Spawn task to receive messages from this client
    let mut recv_task = tokio::spawn(receive_messages(
        receiver,
        tx.clone(),
        room_id,
        user_id,
        username.clone(),
        state.clone(),
    ));

    // Wait for either task to complete
    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
        },
        _ = &mut recv_task => {
            send_task.abort();
        },
    }

    tracing::info!("WebSocket connection closed for user {} in room {}", username, room_id);
}

async fn send_messages(
    mut sender: SplitSink<WebSocket, Message>,
    mut rx: broadcast::Receiver<String>,
) {
    while let Ok(msg) = rx.recv().await {
        if sender.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}

async fn receive_messages(
    mut receiver: SplitStream<WebSocket>,
    tx: broadcast::Sender<String>,
    room_id: i32,
    user_id: i32,
    username: String,
    state: AppState,
) {
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            // Parse incoming message
            if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                match ws_msg {
                    WsMessage::Message { content } => {
                        // Save message to database
                        let create_req = CreateMessageRequest {
                            content: content.clone(),
                        };

                        if let Err(e) = state
                            .message_service
                            .create_message(user_id, room_id, create_req)
                            .await
                        {
                            tracing::error!("Failed to save message: {:?}", e);
                            continue;
                        }

                        // Broadcast to all clients in the room
                        let broadcast = WsBroadcast {
                            msg_type: "message".to_string(),
                            sender: username.clone(),
                            sender_id: user_id,
                            content,
                        };

                        if let Ok(broadcast_json) = serde_json::to_string(&broadcast) {
                            let _ = tx.send(broadcast_json);
                        }
                    }
                }
            }
        }
    }
}
