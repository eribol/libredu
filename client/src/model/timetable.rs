use serde::*;
use crate::model;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Day{
    pub id: i32,
    pub name: String
}

impl Day{
    pub fn new() -> Vec<Self>{
        vec![
            Self {
                id: 1,
                name: "Pazartesi".parse().unwrap()
            },
            Self {
                id: 2,
                name: "Salı".parse().unwrap()
            },
            Self {
                id: 3,
                name: "Çarşamba".parse().unwrap()
            },
            Self {
                id: 4,
                name: "Perşembe".to_string()
            },
            Self {
                id: 5,
                name: "Cuma".to_string()
            },
            Self {
                id: 6,
                name: "Cumartesi".to_string()
            },
            Self {
                id: 7,
                name: "Pazar".to_string()
            },
        ]

    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClassAvailable{
    pub hours: Vec<bool>,
    pub(crate) day: Day
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsertClassAvailable{
    pub class_id: i32,
    pub hours: Vec<bool>,
    pub(crate) day: i32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewTimetable {
    pub class_id: i32,
    pub day_id: i32,
    pub hour: i16,
    pub activities: i32
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct Params{
    hour: i32,
    depth: usize,
    depth2: usize
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct NewClassTimetable {
    pub class_id: Option<i32>,
    pub day_id: Option<i32>,
    pub hour: Option<i16>,
    pub activities: Option<i32>
}



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Activity{
    pub(crate) id: i32,
    pub(crate) subject: i32,
    pub(crate) teacher: Option<i32>,
    //pub(crate) class: i32,
    pub(crate) hour: i16,
    pub(crate) split: bool,
    pub(crate) classes: Vec<i32>,
}
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Class{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub school: i32,
    pub teacher: Option<i32>,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TimetableData{
    pub(crate) tat: Vec<model::teacher::TeacherAvailableForTimetables>,
    pub(crate) cat: Vec<ClassAvailableForTimetable>,
    pub(crate) acts: Vec<Activity>,
    classes: Vec<Class>,
    teachers: Vec<model::teacher::Teacher>,
    timetables: Vec<NewClassTimetable>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClassAvailableForTimetable{
    pub(crate) class_id: i32,
    pub(crate) day: i32,
    pub(crate) hours: Vec<bool>
}