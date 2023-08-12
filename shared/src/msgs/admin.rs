use moonlight::*;

use crate::{School, User, models::{class::{Class, ClassLimitation}, timetables::{Timetable, Activity}, teacher::{Teacher, TeacherLimitation}}};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum AdminUpMsgs{
    GetLastSchools,
    SearchSchool(i32),
    GetClasses(i32),
    GetTimetables(i32),
    GetClassesLimitations(i32),
    GetTeachersLimitations(i32),
    GetActivities(i32),
    GetTeachers(i32),
    UpdateClassLimitations(Vec<ClassLimitation>),
    UpdateTeacherLimitations(Vec<TeacherLimitation>),
    DelAct(i32)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum AdminDownMsgs{
    LastSchools(Vec<AdminSchool>),
    GetSchool(AdminSchool),
    GetClasses(Vec<Class>),
    GetTimetables(Vec<Timetable>),
    GetClassesLimitations(Vec<ClassLimitation>),
    GetTeachersLimitations(Vec<TeacherLimitation>),
    GetActivities(Vec<Activity>),
    GetTeachers(Vec<Teacher>),
    UpdateClassLimitationsError(String),
    UpdateTeacherLimitationsError(String),
    UpdatedClassLimitations,
    UpdatedTeacherLimitations,
    DeletedAct
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct AdminSchool{
    pub school: School,
    pub principle: SchoolManager
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct SchoolManager{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub last_login: NaiveDateTime
}