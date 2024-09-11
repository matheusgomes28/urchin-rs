use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EnumIter,
    PrimaryKeyTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub content: String,
    pub excerpt: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// impl Related<super::fruit::entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Fruit.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}


// Urchin stuff only
// Data that user will pass to the endpoint
#[derive(Deserialize)]
pub struct AddPostRequest {
    pub title: String,
    pub content: String,
    pub excerpt: String,
}

#[derive(Serialize)]
pub struct AddPostResponse {
    pub post_id: i32,
}

#[derive(Serialize)]
pub struct DeletePostResponse {
    pub post_id: i32,
}

#[derive(Clone, Serialize)]
pub struct GetPostResponse
{
    pub post_id: i32,
    pub title: String,
    pub content: String,
    pub excerpt: String,
}
