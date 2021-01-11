use serde::*;
use crate::model::city::{Town,City};
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
    pub school_type: i32,
    pub tel: Option<String>,
    pub location: Option<String>,
    pub city: City,
    pub town: Town
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SchoolTeacher {
    pub(crate) school_id: i32,
    pub(crate) user_id: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NewSchool {
    pub(crate) name: String,
    pub school_type: i32,
    pub city: i32,
    pub town: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SchoolType {
    pub(crate) name: String,
    pub(crate) id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSchoolForm{
    pub(crate) name: String,
    //pub manager: i32,
    //pub school_type: i32,
    pub tel: Option<String>,
    pub location: Option<String>,
    pub city: City,
    pub town: Town
}

impl Default for UpdateSchoolForm{
    fn default()-> Self{
        UpdateSchoolForm{
            name: "".to_string(),
            tel: Some("".to_string()),
            location: Some("".to_string()),
            city: Default::default(),
            town: Default::default()
        }
    }
}