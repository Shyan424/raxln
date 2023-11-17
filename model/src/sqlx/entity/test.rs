use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Test {
    pub id: i32,
    pub key: String,
    pub value: String
}