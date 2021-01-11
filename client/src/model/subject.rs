use serde::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct NewSubject{
    pub name: String,
    pub school: i32,
    pub optional: bool,
    pub kademe: String
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Subject{
    pub id: i32,
    pub name: String,
    pub school: i32,
    pub optional: bool,
    pub kademe: String
}