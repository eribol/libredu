use serde::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserDetail{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: Option<String>,
    pub is_admin: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Teacher{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginForm{
    pub username: String,
    pub password: String
}