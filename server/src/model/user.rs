use sqlx::types::chrono::NaiveDateTime;
use serde::*;


//use crate::request;
//use shared::models::user::AuthUser;
#[derive(Clone, sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub email: String,
    pub password: String,
    pub date_join: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_staff: Option<bool>,
    pub is_admin: bool,
    pub tel: Option<String>,
    pub gender:Option<String>,
    pub img:Option<String>,
}


#[derive(sqlx::FromRow,Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthUser{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: Option<String>,
    pub is_admin: bool,
}

#[derive(sqlx::FromRow,Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimpleUser{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct Teacher{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub role_id: i32,
    pub role_name: String
}

#[derive(sqlx::FromRow,Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimpleTeacher{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct Student{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginForm{
    pub username: String,
    pub password: String
}

#[derive(Clone, sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct SignUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password1: String,
    pub password2: String,
    pub tel: String,
    pub gender:String,
}