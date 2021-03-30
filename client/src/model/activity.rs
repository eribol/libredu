use serde::*;
use crate::model::class;
use crate::model::teacher::{TeacherAvailableForTimetables, Teacher};
use crate::model::class::Class;
use crate::model::subject::Subject;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NewActivity{
    pub(crate) subject: i32,
    pub(crate) teacher: i32,
    pub(crate) hour: String,
    //pub(crate) class: i32,
    pub(crate) split: bool,
    pub(crate) classes: Vec<i32>,
    pub(crate) teachers: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Activity{
    pub(crate) id: i32,
    pub(crate) subject: i32,
    pub(crate) teacher: Option<i32>,
    pub(crate) hour: i16,
    //pub(crate) class: i32,
    pub(crate) split: bool,
    pub(crate) classes: Vec<i32>,
    pub(crate) teachers: Option<Vec<i32>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullActivity{
    pub(crate) id: i32,
    pub(crate) subject: Subject,
    pub(crate) teacher: Option<Teacher>,
    pub(crate) hour: i16,
    //pub(crate) class: i32,
    pub(crate) split: bool,
    pub(crate) classes: Vec<Class>,
    pub(crate) teachers: Option<Vec<Teacher>>
}

