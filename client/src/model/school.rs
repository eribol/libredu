use serde::*;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct School{
    pub id: i32,
    pub name: String,
    pub(crate) manager: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SchoolDetail {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub manager: i32,
    //pub school_type: Option<i32>,
    pub tel: Option<String>,
    pub location: Option<String>,
    //pub city: City,
    //pub town: Town
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SchoolTeacher {
    pub(crate) school_id: i32,
    pub(crate) user_id: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NewSchool {
    pub(crate) name: String,
    //pub school_type: Option<i32>,
    //pub city: i32,
    //pub town: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SchoolType {
    pub(crate) name: String,
    pub(crate) id: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateSchoolForm{
    pub(crate) name: String,
    //pub manager: i32,
    //pub school_type: i32,
    pub tel: Option<String>,
    pub location: Option<String>,
}

impl Default for UpdateSchoolForm{
    fn default()-> Self{
        UpdateSchoolForm{
            name: "".to_string(),
            tel: Some("".to_string()),
            location: Some("".to_string()),
        }
    }
}
use crate::i18n::{I18n, Lang};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SchoolMenu2{
    pub link: String,
    pub name: String,
}

pub(crate) fn create_menu(lang: &I18n) -> Vec<SchoolMenu2>{
    use crate::{create_t, with_dollar_sign};
    create_t![lang];
    vec![
        SchoolMenu2 {
            link: String::from(""),
            name: String::from(t!["homepage"])
        },
        SchoolMenu2 {
            link: String::from("detail"),
            name: String::from(t!["detail-page"])
        },
        SchoolMenu2 {
            link: String::from("students"),
            name: String::from(t!["students"])
        },
        SchoolMenu2 {
            link: String::from("subjects"),
            name: String::from(t!["subjects-page"])
        },
        SchoolMenu2 {
            link: String::from("class_rooms"),
            name: String::from(t!["class_rooms-page"])
        },
    ]
}
