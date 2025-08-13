use std::borrow::Cow;

use crate::{
    app::{
        admin, forget_password,
        login::get_school,
        login_user,
        messages::{last_message, streaming_messages},
        school::classes::{add_class_error, selected_timetable_hour},
        signin::server_error,
    },
    *,
};
use app::school::homepage::school_api;
use shared::{
    msgs::{
        classes::ClassDownMsgs,
        lectures::LecturesDownMsg,
        messages::MessagesDownMsgs,
        teachers::TeacherDownMsgs,
        timetables::{TimetablesDownMsgs, TimetablesUpMsgs},
    },
    DownMsg, UpMsg,
};
use zoon::println;

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, cor_id| {
        //println!("DownMsg received: {:?}", down_msg);

        app::unfinished_mutations().update_mut(|cor_ids| {
            cor_ids.remove(&cor_id);
        });
        match down_msg {
            // ------ Auth ------
            DownMsg::LoginError(error) => {
                println!("Login Error {error}");
                crate::app::login::set_login_error(error);
            }
            DownMsg::LoggedIn(user) => {
                println!("Login");
                get_school();
                crate::app::login::login_error().set(None);
                crate::app::login::set_and_store_logged_user(user)
            }
            DownMsg::LoggedOut => crate::app::on_logged_out_msg(),
            DownMsg::LoggedOutError(_) => (),
            DownMsg::SigninError(e) => server_error().set(Some(Cow::from(e))),
            DownMsg::Signin => {
                println!("Signed");
                crate::app::signin::register().set(true)
            }
            DownMsg::Registered(user) => crate::app::login::set_and_store_logged_user(user),
            DownMsg::ResgiterErrors => println!("Regist error"),
            DownMsg::GetSchool { id, name } => {
                use crate::app::school::{school, School};
                school().set(Some(School { id, name }));
                send_msg(UpMsg::GetSchoolApi(id));
            }
            DownMsg::AddedSchool(school) => local_storage()
                .insert("school", &school)
                .expect("Yüklenemedi"),
            DownMsg::Teachers(t_dmsg) => match t_dmsg {
                TeacherDownMsgs::GetTeachers(mut tchrs) => {
                    tchrs.sort_by(|a, b| a.first_name.cmp(&b.first_name));
                    crate::app::school::teachers::teachers()
                        .lock_mut()
                        .replace_cloned(tchrs.clone());
                }
                TeacherDownMsgs::AddedTeacher(teacher) => {
                    let thcrs = crate::app::school::teachers::teachers().lock_mut().to_vec();
                    let index = thcrs.iter().enumerate().find(|t| t.1.id == teacher.id);
                    match index {
                        Some(i) => crate::app::school::teachers::teachers()
                            .lock_mut()
                            .set_cloned(i.0, teacher),
                        None => crate::app::school::teachers::teachers()
                            .lock_mut()
                            .push_cloned(teacher),
                    }
                }
                TeacherDownMsgs::DeletedTeacher(id) => {
                    if id != login_user().get_cloned().unwrap().id {
                        crate::app::school::teachers::teachers()
                            .lock_mut()
                            .retain(|t| t.id != id);
                    }
                }
                _ => (),
            },
            DownMsg::Lectures(l_msg) => match l_msg {
                LecturesDownMsg::GetLectures(mut lectures) => {
                    lectures.sort_by(|a, b| a.short_name.cmp(&b.short_name));
                    crate::app::school::lectures::lectures()
                        .lock_mut()
                        .replace_cloned(lectures);
                }
                LecturesDownMsg::AddedLecture(lecture) => {
                    let lecs = crate::app::school::lectures::lectures().lock_mut().to_vec();
                    let index = lecs.iter().enumerate().find(|l| l.1.id == lecture.id);
                    match index {
                        Some(i) => crate::app::school::lectures::lectures()
                            .lock_mut()
                            .set_cloned(i.0, lecture),
                        None => crate::app::school::lectures::lectures()
                            .lock_mut()
                            .push_cloned(lecture),
                    }
                }
                LecturesDownMsg::DeletedLecture(id) => {
                    crate::app::school::lectures::lectures()
                        .lock_mut()
                        .retain(|t| t.id != id);
                }
                _ => (),
            },
            DownMsg::Classes(msg) => {
                match msg {
                    ClassDownMsgs::GetClasses(mut classes) => {
                        classes.sort_by(|a, b| a.label().cmp(&b.label()));
                        crate::app::school::classes::classes()
                            .lock_mut()
                            .replace_cloned(classes.clone())
                    }
                    ClassDownMsgs::AddedClass(class) => app::school::classes::classes()
                        .lock_mut()
                        .push_cloned(class.clone()),
                    ClassDownMsgs::DeletedClass(id) => {
                        crate::app::school::classes::classes()
                            .lock_mut()
                            .retain(|t| t.id != id);
                    }
                    ClassDownMsgs::AddClassError(e) => add_class_error().set(Some(e)),
                    _ => (),
                };
                //crate::app::school::classes::create_chunks()
            }
            DownMsg::Timetables(msg) => {
                match msg {
                    TimetablesDownMsgs::GetTimetables(timetables) => {
                        crate::app::school::classes::timetables()
                            .lock_mut()
                            .replace_cloned(timetables.clone());
                        if let Some(group) = timetables.get(0) {
                            crate::app::school::classes::change_timetable(group.id.to_string());
                        }
                    }
                    TimetablesDownMsgs::AddedTimetable(timetable) => {
                        crate::app::school::classes::timetables()
                            .lock_mut()
                            .push_cloned(timetable)
                    }
                    TimetablesDownMsgs::DeletedTimetable(id) => {
                        crate::app::school::classes::timetables()
                            .lock_mut()
                            .retain(|t| t.id != id);
                    }
                    TimetablesDownMsgs::GetSchedules(mut schedules) => {
                        if schedules.starts.len() != selected_timetable_hour().lock_mut().len() {
                            schedules.starts = vec![
                                NaiveTime::parse_from_str("00:00", "%H:%M")
                                    .unwrap();
                                selected_timetable_hour().lock_mut().len()
                            ];
                            send_msg(UpMsg::Timetables(TimetablesUpMsgs::UpdateSchedules(
                                schedules.clone(),
                            )));
                        }
                        if schedules.ends.len() != selected_timetable_hour().lock_mut().len() {
                            schedules.ends = vec![
                                NaiveTime::parse_from_str("00:00", "%H:%M")
                                    .unwrap();
                                selected_timetable_hour().lock_mut().len()
                            ];
                            send_msg(UpMsg::Timetables(TimetablesUpMsgs::UpdateSchedules(
                                schedules.clone(),
                            )));
                        }
                        crate::app::school::timetables::timetable_schedules().set(Some(schedules));
                    }
                    _ => (),
                };
            }
            DownMsg::ResetPassword => forget_password::is_sent().set(true),
            DownMsg::Admin(a_msg) => admin::msgs::get_msg(a_msg),
            DownMsg::Messages(msg) => match msg {
                MessagesDownMsgs::GetMessages(msgs) => {
                    super::app::messages::msgs()
                        .lock_mut()
                        .replace_cloned(msgs.clone());
                    let last_m = msgs.last();
                    if let Some(m) = last_m {
                        last_message().set(Some(m.id));
                        streaming_messages();
                    }
                }
                MessagesDownMsgs::GetNewMessages(msgs) => {
                    let last_m = msgs.last();
                    if let Some(m) = last_m {
                        use zoon::println;
                        println!("{}", m.id);
                        last_message().set(Some(m.id));
                    }
                    for m in msgs {
                        super::app::messages::msgs().lock_mut().push_cloned(m);
                    }
                }
                MessagesDownMsgs::SentMessage(msg) => {
                    super::app::messages::msgs().lock_mut().push_cloned(msg);
                }
                _ => (),
            },
            DownMsg::GetSchoolApi(api) => {
                school_api().set(Some(api));
            }
            _ => (),
        }
    })
    .auth_token_getter(app::auth_token)
}

pub fn send_msg(msg: UpMsg) {
    Task::start(async {
        match connection().send_up_msg(msg).await {
            Err(_error) => {}
            Ok(_msg) => {
                //println!("error occured")
            }
        }
    });
}
