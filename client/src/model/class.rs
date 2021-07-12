use serde::*;
use crate::model::subject::Subject;
use crate::model::teacher::Teacher;
use crate::model::student::{SimpleStudent};
use crate::model::timetable::Day;
use crate::model::activity;
use crate::i18n::I18n;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Class{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub school: i32,
    pub group_id: i32
}
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassAvailable{
    pub hours: Vec<bool>,
    pub(crate) day: Day
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ClassTimetable{
    pub id: i32,
    pub class_id: i32,
    pub day_id: i32,
    pub hour: i16,
    pub subject: String,
    pub activity: ClassTimetableActivity
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ClassTimetableActivity{
    pub id: i32,
    pub teachers: Vec<Teacher>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NewClass{
    pub kademe: String,
    pub sube: String,
    pub group_id: i32
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct UpdateClass{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub group_id: i32
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ClassContext{
    pub class: Class,
    pub students: Option<Vec<SimpleStudent>>,
    pub activities: Option<Vec<activity::FullActivity>>,
    pub limitations: Option<Vec<ClassAvailable>>,
    pub timetables: Option<Vec<ClassTimetable>>
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ClassMenu{
    pub link: String,
    pub name: String
}

pub fn create_menu(lang: &I18n) -> Vec<ClassMenu>{
    use crate::{create_t, with_dollar_sign};
    create_t![lang];
    vec![
        ClassMenu {
            link: String::from(""),
            name: String::from("Sınıf".to_string() + " " + &t!["info"]),
        },
        /*
        ClassMenu {
            link: String::from("students"),
            name: String::from(t!["students"]),
        },
        */
        ClassMenu {
            link: String::from("activities"),
            name: String::from(t!["activities"]),
        },
        ClassMenu {
            link: String::from("limitations"),
            name: String::from(t!["limitations"]),
        },
        ClassMenu {
            link: String::from("timetables"),
            name: String::from(t!["timetables"]),
        }
        /*

        ClassMenu {
            link: "timetables",
            name: "Ders Programı",
        }

         */
    ]
}

impl ClassContext{
    pub fn get_mut_students(&mut self) -> &mut Vec<SimpleStudent>{
        self.students.get_or_insert(vec![])
    }
    pub fn get_students(&self) -> &Vec<SimpleStudent>{
        self.students.as_ref().unwrap()
    }
}