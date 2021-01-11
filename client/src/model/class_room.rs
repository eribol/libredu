use serde::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewClassroom{
    pub name: String,
    pub school: i32,
    pub rw: i16,
    pub cl: i16,
    pub width: i16
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Classroom{
    pub id: i32,
    pub name: String,
    pub school: i32,
    pub rw: i16,
    pub cl: i16,
    pub width: i16
}