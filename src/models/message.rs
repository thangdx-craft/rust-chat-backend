use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    
    pub sender_id: i32,
    
    pub room_id: i32,
    
    pub content: String,
    
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SenderId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::room::Entity",
        from = "Column::RoomId",
        to = "super::room::Column::Id"
    )]
    Room,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: i32,
    pub sender_id: i32,
    pub room_id: i32,
    pub content: String,
    pub created_at: DateTime,
}

impl From<Model> for MessageResponse {
    fn from(message: Model) -> Self {
        MessageResponse {
            id: message.id,
            sender_id: message.sender_id,
            room_id: message.room_id,
            content: message.content,
            created_at: message.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
}
