use moonlight::*;

use crate::{School, User};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum AdminUpMsgs{
    GetLastSchools,
    SearchSchool(i32)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum AdminDownMsgs{
    LastSchools(Vec<AdminSchool>),
    GetSchool(AdminSchool)
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