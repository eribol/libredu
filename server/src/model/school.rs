use serde::*;
use crate::model::city::{City, Town};

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct School{
    pub(crate) id: i32,
    name: String,
    pub(crate) manager: i32
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct SchoolDetail {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub manager: i32,
    pub school_type: i32,
    pub tel: Option<String>,
    pub location: Option<String>,
    pub city: City,
    pub town: Town
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct SchoolTeacher {
    pub(crate) school_id: i32,
    pub(crate) user_id: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NewSchool {
    pub(crate) name: String,
    pub school_type: i32,
    pub city: i32,
    pub town: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, sqlx::FromRow)]
pub struct SchoolType {
    pub(crate) name: String,
    pub(crate) id: i32
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateSchoolForm{
    pub(crate) name: String,
    pub(crate) tel: Option<String>,
    pub(crate) location: Option<String>
}