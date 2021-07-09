use serde::*;
use crate::model::class::Class;
use crate::model::subject::Subject;
use crate::model::teacher::Teacher;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewActivity{
    pub(crate) subject: i32,
    pub(crate) hour: String,
    pub(crate) split: bool,
    pub(crate) classes: Vec<i32>,
    pub(crate) teachers: Vec<i32>
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Activity{
    pub(crate) id: i32,
    pub(crate) subject: i32,
    pub(crate) hour: i16,
    pub(crate) split: bool,
    pub(crate) classes: Vec<i32>,
    pub(crate) teachers: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FullActivity{
    pub(crate) id: i32,
    pub(crate) subject: Subject,
    pub(crate) hour: i16,
    //pub(crate) class: i32,
    pub(crate) split: bool,
    pub(crate) classes: Vec<Class>,
    pub(crate) teachers: Vec<Teacher>,
}