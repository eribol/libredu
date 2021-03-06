use serde::*;
use crate::model::student::Student;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Library {
    pub id: i32,
    pub school: i32,
    pub manager: i32,
    pub barkod_min: i32,
    pub barkod_max: i32,
    pub student: i32
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NewLibrary {
    pub school: i32,
    pub manager: i32,
    pub barkod_min: i32,
    pub barkod_max: i32,
    pub student: i32
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LibraryContext {
    pub library: Library,
    pub books: Vec<Book>,
    pub students: Vec<Student>
}