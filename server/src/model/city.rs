use serde::*;

#[derive(sqlx::FromRow,Debug, Serialize, Deserialize, Clone, Default)]
pub struct City{
    pub pk: i32,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Default, Clone)]
pub struct Town{
    pub city: i32,
    pub pk: i32,
    pub name: String,
}