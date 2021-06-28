use serde::*;
use crate::model::class::{Class, ClassContext};
use crate::model::{timetable, activity};

#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TeacherAvailable{
    pub group_id: Option<i32>,
    pub day: timetable::Day,
    pub hours: Vec<bool>
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TeacherTimetable{
    pub id: i32,
    pub class_id: Vec<ClassContext>,
    pub day_id: i32,
    pub hour: i16,
    pub subject: String,
}
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TeacherTimetable2{
    pub id: i32,
    pub class_id: Vec<Class>,
    pub day_id: i32,
    pub hour: i16,
    pub subject: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TeacherAvailableForTimetables{
    pub(crate) user_id: i32,
    school_id: i32,
    pub(crate) day: i32,
    pub(crate) hours: Vec<bool>
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Teacher{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub role_id: i32,
    pub role_name: String,
    pub is_active: bool,
    pub email: Option<String>,
    pub tel: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TeacherContext{
    pub teacher: Teacher,
    pub group: Vec<TeacherGroupContext>,
    pub activities: Option<Vec<activity::FullActivity>>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TeacherGroupContext{
    pub group: i32,
    pub activities: Option<Vec<activity::FullActivity>>,
    pub limitations: Option<Vec<TeacherAvailable>>,
    pub timetables: Option<Vec<TeacherTimetable2>>
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TeacherMenu<'a>{
    pub link: &'a str,
    pub name: &'a str,
}

pub const TEACHER_MENU: &[TeacherMenu] = &[
    TeacherMenu {
        link: "",
        name: "Öğretmen Bilgileri",
    },
    TeacherMenu {
        link: "activities",
        name: "Aktiviteler",
    },
    TeacherMenu {
        link: "limitations",
        name: "Kısıtlamalar",
    },
    TeacherMenu {
        link: "timetables",
        name: "Ders Programı",
    }
];