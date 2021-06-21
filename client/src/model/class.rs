use serde::*;
use crate::model::subject::Subject;
use crate::model::teacher::Teacher;
use crate::model::student::{SimpleStudent};
use crate::model::timetable::Day;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Class{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub school: i32,
    pub group_id: i32
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClassAvailable{
    pub hours: Vec<bool>,
    pub(crate) day: Day
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ClassTimetable{
    pub id: i32,
    pub class_id: i32,
    pub day_id: i32,
    pub hour: i16,
    pub subject: String,
    pub activity: ClassTimetableActivity
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ClassTimetableActivity{
    pub id: i32,
    pub teacher: Teacher
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ClassActivity{
    pub id: i32,
    pub subject: Subject,
    pub teacher: Teacher,
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClassContext{
    pub class: Class,
    pub students: Option<Vec<SimpleStudent>>,
    pub activities: Option<Vec<ClassActivity>>,
    pub limitations: Option<Vec<ClassAvailable>>,
    pub timetables: Option<Vec<ClassTimetable>>
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ClassMenu<'a>{
    pub link: &'a str,
    pub name: &'a str,
}

pub const CLASS_MENU: &[ClassMenu] = &[

    ClassMenu {
        link: "",
        name: "Sınıf Bilgileri",
    },
    ClassMenu {
        link: "students",
        name: "Öğrenciler",
    },
    ClassMenu {
        link: "activities",
        name: "Aktiviteler",
    },
    ClassMenu {
        link: "limitations",
        name: "Kısıtlamalar",
    },
    ClassMenu {
        link: "timetables",
        name: "Ders Programı",
    }
];

impl ClassContext{
    pub fn get_mut_students(&mut self) -> &mut Vec<SimpleStudent>{
        self.students.get_or_insert(vec![])
    }
    pub fn get_students(&self) -> &Vec<SimpleStudent>{
        self.students.as_ref().unwrap()
    }
}