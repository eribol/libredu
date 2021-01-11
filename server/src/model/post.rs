use serde::*;
use sqlx::types::chrono;
use crate::model::school::SchoolDetail;


#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow, Default)]
pub struct NewPost{
    pub body: String,
    pub sender: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post{
    pub(crate) id: i32,
    pub(crate) body: String,
    pub(crate) pub_date: chrono::DateTime<chrono::Utc>,
    pub(crate) school: Option<i32>,
    pub(crate) sender: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SchoolPost{
    pub(crate) id: i32,
    pub(crate) body: String,
    pub(crate) pub_date: chrono::DateTime<chrono::Utc>,
    pub(crate) school: Option<SchoolDetail>,
    pub(crate) sender: i32
}