use crate::models::timetables::*;
use moonlight::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum TimetablesUpMsgs {
    GetTimetable,
    AddTimetable(AddTimetable),
    DelTimetable(i32),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum TimetablesDownMsgs {
    GetTimetables(Vec<Timetable>),
    GetTimetablesError(String),
    AddedTimetable(Timetable),
    AddTimetableError(String),
    DeletedTimetable(i32),
    DeleteTimetableError(String),
}