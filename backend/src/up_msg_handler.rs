use crate::{
    connection::{
        self, get_user,
        school::{add_school, get_school, update_school}, forget_password, reset_password,
    },
    user::{self, is_user_exist, get_user_with_id},
};
use moon::*;
use shared::{
    msgs::{
        classes::ClassUpMsgs,
        teachers::TeacherUpMsgs, lectures::LecturesUpMsg, timetables::TimetablesUpMsgs,
    },
    *,
};

use self::{admin::admin_msgs, classes::classes_msgs};
pub mod auth;
pub mod classes;
pub mod lectures;
pub mod school;
pub mod teachers;
pub mod timetables;
pub mod admin;
pub mod messages;

pub async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    let UpMsgRequest {
        up_msg,
        cor_id,
        session_id,
        auth_token,
    } = req;

    let down_msg = match up_msg {
        // ------ Auth ------
        UpMsg::Login { email, password } => {
            let user = user::login(email, password);
            match user.await {
                Ok(u) => {
                    let auth_token = AuthToken::new(format!("{}:{}", u.id, EntityId::new()));
                    //println!("{:?}", &auth_token.clone().into_string());
                    connection::set_user(u.id, &auth_token)
                        .await
                        .expect("Not set user");
                    let user2 = User {
                        id: u.id,
                        first_name: u.first_name,
                        auth_token,
                        is_admin: u.is_admin,
                        is_active: u.is_active
                    };

                    DownMsg::LoggedIn(user2)
                }
                Err(_e) => DownMsg::LoginError("Sorry, invalid name or password.".to_owned()),
            }
        }
        UpMsg::Logout => {
            let auth = auth_token.unwrap().into_string();
            let u: Result<i32, redis::RedisError> = connection::get_user(&auth).await;
            match u {
                Ok(id) => match connection::del_user(id, auth).await {
                    Ok(_) => DownMsg::LoggedOut,
                    Err(e) => DownMsg::LoggedOutError(e.to_string()),
                },
                Err(e) => DownMsg::LoggedOutError(e.to_string()),
            }
        }
        UpMsg::ForgetPassword { email } => {
            match is_user_exist(&email).await{
                Ok(_) => forget_password::forget_password(email).await,
                Err(_) => DownMsg::ResetPassword
            }
        },
        UpMsg::ResetPassword(form) => {
            reset_password::reset_password(form).await
        },
        UpMsg::Signin { form } => {
            match is_user_exist(&form.email).await{
                Ok(_) => DownMsg::SigninError("This email is registered".to_string()),
                Err(_) => {
                    let auth_token = AuthToken::new(EntityId::new());
                    
                    connection::register(form, &auth_token).await
                }
            }
        },
        UpMsg::Register(token, email) => {
            
            connection::get_register(token, email).await
        },
        UpMsg::AddSchool { name } => add_school(auth_token, name).await,
        UpMsg::GetSchool => {
            if let Some(auth_token) = auth_token {
                let manager = get_user(&auth_token.into_string()).await;
                if let Ok(school) = get_school(manager.unwrap()).await {
                    DownMsg::GetSchool {
                        id: school.id,
                        name: school.name,
                    }
                } else {
                    DownMsg::LoginError("Server error".to_string())
                }
            } else {
                DownMsg::LoginError("Not logged".to_string())
            }
        }
        UpMsg::UpdateSchool(form) => update_school(auth_token, &form).await,
        UpMsg::Classes(class_msg) => {
            classes_msgs(class_msg, auth_token).await
        }
        UpMsg::Teachers(t_msg) => {
            if let Some(auth) = auth_token {
                let manager = get_user(&auth.into_string()).await;
                let school_msg = crate::up_msg_handler::school::get_school(manager.unwrap()).await;
                if let DownMsg::GetSchool { id, .. } = school_msg {
                    use self::teachers::*;
                    match t_msg {
                        TeacherUpMsgs::GetTeachers => get_teachers(id).await,
                        TeacherUpMsgs::AddTeacher(form) => add_teacher(id, form).await,
                        TeacherUpMsgs::DelTeacher(user_id) => del_teacher(user_id, id).await,
                        TeacherUpMsgs::UpdateTeacher(form) => update_teacher(form).await,
                    }
                } else {
                    school_msg
                }
            } else {
                DownMsg::AuthError("Not Auth".to_string())
            }
        }
        UpMsg::Timetables(tt_msg) => {
            if let Some(auth) = auth_token {
                let manager = get_user(&auth.into_string()).await;
                let school_msg = crate::up_msg_handler::school::get_school(manager.unwrap()).await;
                if let DownMsg::GetSchool { id, .. } = school_msg {
                    match tt_msg {
                        TimetablesUpMsgs::GetTimetable => timetables::get_class_groups(id).await,
                        TimetablesUpMsgs::AddTimetable(form) => timetables::add_timetable(form, id).await,
                        TimetablesUpMsgs::DelTimetable(group_id) => timetables::del_timetable(id, group_id).await,
                        TimetablesUpMsgs::GetSchedules(group_id) => timetables::get_schedules(group_id).await,
                        TimetablesUpMsgs::UpdateSchedules(schedules) => timetables::update_schedules(schedules).await,
                        TimetablesUpMsgs::UpdateTimetable(form) => timetables::update_timetable(form, id).await,
                        //TeacherUpMsgs::DelTeacher(user_id) => del_teacher(user_id, id).await
                    }
                } else {
                    school_msg
                }
            } 
            else {
                DownMsg::AuthError("Not Auth".to_string())
            }
        }
        UpMsg::Lectures(l_msg) =>{
            if let Some(auth) = auth_token {
                let manager = get_user(&auth.into_string()).await;
                let school_msg = crate::up_msg_handler::school::get_school(manager.unwrap()).await;
                if let DownMsg::GetSchool { id, .. } = school_msg {
                    match l_msg {
                        LecturesUpMsg::GetLectures => lectures::get_lectures(id).await,
                        LecturesUpMsg::AddLecture(form) => lectures::add_lecture(id, form).await,
                        LecturesUpMsg::UpdateLecture(form) => lectures::update_lecture(id, form).await,
                        LecturesUpMsg::DelLecture(l_id) => lectures::del_lecture(l_id, id).await,
                    }
                }    
                else {
                    school_msg
                }
            } 
            else {
                DownMsg::AuthError("Not Auth".to_string())
            }
        }
        UpMsg::Admin(a_msg)=>{
            admin_msgs(a_msg, auth_token).await
        }
        UpMsg::Messages(msg)=>{
            messages::message(msg, auth_token).await
        }
    };
    if let Some(session) = sessions::by_session_id().wait_for(session_id).await {
        session.send_down_msg(&down_msg, cor_id).await;
    } else {
        eprintln!("cannot find the session with id `{}`", session_id);
    }
}
