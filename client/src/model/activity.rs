use serde::*;
use crate::model::class;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Subject{
    pub name: String,
    pub kademe: String,
    pub optional: bool,
    pub id: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NewActivity{
    pub(crate) subject: i32,
    pub(crate) teacher: i32,
    pub(crate) hour: String,
    //pub(crate) class: i32,
    pub(crate) split: bool,
    pub(crate) classes: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Activity{
    pub(crate) id: i32,
    pub(crate) subject: i32,
    pub(crate) teacher: i32,
    pub(crate) hour: i16,
    //pub(crate) class: i32,
    pub(crate) split: bool,
    pub(crate) classes: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeacherActivity{
    pub(crate) id: i32,
    pub(crate) subject: Subject,
    pub(crate) teacher: Option<i32>,
    pub(crate) hour: i16,
    //pub(crate) class: class::Class,
    pub(crate) classes: Vec<class::Class>,
    pub(crate) split: bool
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ActivityTeacher{
    pub(crate) id: i32,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
}