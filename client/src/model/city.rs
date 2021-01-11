use serde::*;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct City{
    pub pk: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Town{
    pub city: i32,
    pub pk: i32,
    pub name: String,
}