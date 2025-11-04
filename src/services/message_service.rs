use crate::errors::Result;
use crate::models::message::{self, CreateMessageRequest, MessageResponse};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

#[derive(Clone)]
pub struct MessageService {
    db: DatabaseConnection,
}

impl MessageService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_message(
        &self,
        sender_id: i32,
        room_id: i32,
        req: CreateMessageRequest,
    ) -> Result<MessageResponse> {
        let new_message = message::ActiveModel {
            sender_id: Set(sender_id),
            room_id: Set(room_id),
            content: Set(req.content),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let message = new_message.insert(&self.db).await?;

        Ok(message.into())
    }
}
