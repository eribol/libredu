use crate::models::timetables::*;
use moonlight::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum TimetablesUpMsgs {
    GetTimetable,
    AddTimetable(AddTimetable),
    UpdateTimetable(Timetable),
    DelTimetable(i32),
    GetSchedules(i32),
    UpdateSchedules(TimetableSchedules)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum TimetablesDownMsgs {
    GetTimetables(Vec<Timetable>),
    GetTimetablesError(String),
    AddedTimetable(Timetable),
    AddTimetableError(String),
    UpdatedTimetable(Timetable),
    UpdatedTimetableError(String),
    DeletedTimetable(i32),
    DeleteTimetableError(String),
    GetSchedules(TimetableSchedules)
}