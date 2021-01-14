use serde::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NewStudent{
    pub first_name: String,
    pub last_name: String,
    pub number: i32,
}

#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Student{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub school: i32,
    pub school_number: i32,
}

#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct SimpleStudent{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}