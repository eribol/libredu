use serde::*;
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Library {
    pub id: i32,
    pub school: i32,
    pub manager: i32,
    pub barkod_min: i32,
    pub barkod_max: i32,
    pub student: i32
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewLibrary {
    pub school: i32,
    pub manager: i32,
    pub barkod_min: i32,
    pub barkod_max: i32,
    pub student: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, sqlx::FromRow)]
pub struct Book {
    pub id: i32,
    pub library: i32,
    pub name: String,
    pub writer: String,
    pub piece: i32,
    pub barkod: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NewBook {
    //pub school: i32,
    pub name: String,
    pub writer: String,
    pub piece: i32,
    pub barkod: i32
}