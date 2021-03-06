use serde::*;
use crate::model;

#[derive(Clone, sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Day{
    pub id: i32,
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassAvailable{
    pub hours: Vec<bool>,
    pub(crate) day: Day
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct InsertClassAvailable{
    pub class_id: i32,
    pub hours: Vec<bool>,
    pub(crate) day: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewTimetable {
    pub class_id: i32,
    pub day_id: i32,
    pub hour: i16,
    pub activities: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Timetable {
    pub class_id: i32,
    pub day_id: i32,
    pub hour: i16,
    pub activities: i32
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct Activity{
    id: i32,
    subject: i32,
    teacher: Option<i32>,
    //class: i32,
    hour: i16,
    split: bool,
    classes: Vec<i32>
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Class{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub school: i32,
    pub teacher: Option<i32>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimetableData{
    pub tat: Vec<model::teacher::TeacherAvailableForTimetables>,
    pub cat: Vec<model::class::ClassAvailableForTimetables>,
    pub acts: Vec<Activity>,
    pub classes: Vec<model::class::ClassForTimetables>,
    pub teachers: Vec<model::teacher::Teacher>,
    pub timetables: Vec<NewTimetable>
}

#[derive(sqlx::FromRow,Debug, Clone, Default, Serialize, Deserialize)]
pub struct Teacher{
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub is_admin: bool,
}