use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    
    #[sea_orm(unique)]
    pub email: String,
    
    pub password_hash: String,
    
    pub username: String,
    
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::message::Entity")]
    Messages,
    #[sea_orm(has_many = "super::room_member::Entity")]
    RoomMembers,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl Related<super::room_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoomMembers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// DTOs for API requests/responses
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub username: String,
}

impl From<Model> for UserResponse {
    fn from(user: Model) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
        }
    }
}
