use crate::models::teacher::*;
use moonlight::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum TeacherUpMsgs {
    GetTeachers,
    AddTeacher(AddTeacher),
    DelTeacher(i32),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum TeacherDownMsgs {
    GetTeachers(Vec<Teacher>),
    GetTeachersError(String),
    AddedTeacher(Teacher),
    AddTeacherError(String),
    DeletedTeacher(i32),
    DelTeacherError(String)
}
