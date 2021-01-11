use serde::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Day{
    pub id: i32,
    pub name: String
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