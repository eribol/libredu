use moonlight::*;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct AddTimetable {
    pub name: String,
    pub hour: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Timetable {
    pub id: i32,
    pub name: String,
    pub hour: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct TimetableSchedules {
    pub group_id: i32,
    pub starts: Vec<NaiveTime>,
    pub ends: Vec<NaiveTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Activity {
    pub id: i32,
    pub lecture: i32,
    pub hour: i16,
    pub classes: Vec<i32>,
    pub teachers: Vec<i32>,
    pub no_limit: bool,
    pub partners: Vec<i32>,
    pub classrooms: Vec<i32>,
}
