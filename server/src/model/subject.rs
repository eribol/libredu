use serde::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NewSubject{
    pub name: String,
    pub school: i32,
    pub optional: bool,
    pub kademe: String
}

#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subject{
    pub id: i32,
    pub name: String,
    pub school: i32,
    pub optional: bool,
    pub kademe: String
}