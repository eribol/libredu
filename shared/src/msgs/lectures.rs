use crate::models::lectures::*;
use moonlight::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum LecturesUpMsg{
    GetLectures,
    AddLecture(AddLecture),
    DelLecture(i32),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum LecturesDownMsg{
    GetLectures(Vec<Lecture>),
    GetLectureError(String),
    AddedLecture(Lecture),
    AddLectureError(String),
    DeletedLecture(i32),
    DelLectureError(String)
}