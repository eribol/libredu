use serde::*;
use crate::model::activity::{ActivityTeacher, Subject};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct Class{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub school: i32,
    pub group_id: i32
}
#[derive(Deserialize, Serialize, sqlx::FromRow, Debug, PartialEq, Clone)]
pub struct ClassForTimetables{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub school: i32,
    pub teacher: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct ClassTimetable{
    pub id: i32,
    pub class_id: i32,
    pub day_id: i32,
    pub hour: i16,
    pub subject: String,
    pub activity: ClassTimetableActivity
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct ClassTimetableActivity{
    pub id: i32,
    pub teacher: ActivityTeacher
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct ClassActivity{
    pub id: i32,
    pub subject: Subject,
    pub teacher: ActivityTeacher,
    pub hour: i16,
    pub split: bool
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NewClass{
    pub kademe: String,
    pub sube: String,
    pub group_id: i32
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateClass{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub group_id: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClassAvailableForTimetables{
    pub(crate) class_id: i32,
    pub(crate) day: i32,
    pub(crate) hours: Vec<bool>
}