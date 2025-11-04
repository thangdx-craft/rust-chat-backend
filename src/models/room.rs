use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "rooms")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    
    pub name: String,
    
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomResponse {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime,
}

impl From<Model> for RoomResponse {
    fn from(room: Model) -> Self {
        RoomResponse {
            id: room.id,
            name: room.name,
            created_at: room.created_at,
        }
    }
}
