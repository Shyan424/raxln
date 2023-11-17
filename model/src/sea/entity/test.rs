use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "test")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub key: String,
    pub value: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}