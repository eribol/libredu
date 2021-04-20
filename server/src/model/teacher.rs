use serde::*;
use crate::model::timetable;
use crate::model::class::Class;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeacherAvailable{
    pub group_id: Option<i32>,
    pub day: timetable::Day,
    pub hours: Vec<bool>
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct TeacherAvailableForTimetables{
    user_id: i32,
    school_id: i32,
    pub(crate) day: i32,
    pub(crate) hours: Vec<bool>
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TeacherTimetable{
    pub id: i32,
    pub class_id: Vec<Class>,
    pub day_id: i32,
    pub hour: i16,
    pub subject: String,
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct Teacher{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub role_id: i32,
    pub role_name: String,
    pub is_active: bool
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct SimpleTeacher{
    pub id: i32
}
/*
impl Teacher{
    pub async fn del(&self, _req: &tide::Request<AppState>) -> i32{
        0
    }

    pub async fn del_activities(&self, _req: &tide::Request<AppState>) -> sqlx_core::Result<i32>{
        Ok(0)
    }

}
*/