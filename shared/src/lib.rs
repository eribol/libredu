use models::{
    lectures::{AddLecture, Lecture},
    school::FullSchool,
    timetables::{AddTimetable, Timetable}, users::ResetForm,
};
use moonlight::*;
use msgs::{
    classes::{ClassDownMsgs, ClassUpMsgs},
    teachers::{TeacherDownMsgs, TeacherUpMsgs}, lectures::{LecturesUpMsg, LecturesDownMsg}, timetables::{TimetablesUpMsgs, TimetablesDownMsgs},
};
pub mod models;
pub mod msgs;
pub mod signin;
pub type UserId = EntityId;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub auth_token: AuthToken,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct School {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum UpMsg {
    // ------ Auth ------
    Login { email: String, password: String },
    ForgetPassword{email: String},
    ResetPassword(ResetForm),
    Signin { form: signin::SigninForm },
    Logout,
    AddSchool { name: String },
    Register(String, String),
    UpdateSchool(FullSchool),
    GetSchool,
    Classes(ClassUpMsgs),
    Teachers(TeacherUpMsgs),
    Lectures(LecturesUpMsg),
    Timetables(TimetablesUpMsgs)
}

impl UpMsg {
    pub fn create_downmsg(&self, f: fn() -> DownMsg) -> DownMsg {
        match self {
            Self::Logout => DownMsg::LoggedOut,
            _ => f(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {
    // ------ Auth ------
    LoginError(String),
    LoggedIn(User),
    SigninError(String),
    Signin,
    Register,
    ResetPassword,
    LoggedOut,
    LoggedOutError(String),
    Registered(User),
    ResgiterErrors,
    GetSchool { id: i32, name: String },
    AddedSchool(School),
    AddSchoolError(String),
    UpdateSchool,
    Auth(i32),
    AuthError(String),
    Classes(ClassDownMsgs),
    Teachers(TeacherDownMsgs),
    Lectures(LecturesDownMsg),
    Timetables(TimetablesDownMsgs)
}
