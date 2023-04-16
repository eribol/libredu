use crate::{app::login::get_school, *};
use shared::{msgs::{classes::ClassDownMsgs, teachers::TeacherDownMsgs, lectures::LecturesDownMsg, timetables::TimetablesDownMsgs}, DownMsg, UpMsg};
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
            DownMsg::LoginError(error) => crate::app::login::set_login_error(error),
            DownMsg::LoggedIn(user) => {
                get_school();
                crate::app::login::set_and_store_logged_user(user)
            }
            DownMsg::LoggedOut => crate::app::on_logged_out_msg(),
            DownMsg::LoggedOutError(_) => (),
            DownMsg::SigninError(_e) => (),
            DownMsg::Signin => crate::app::signin::register().set(true),
            DownMsg::Registered(user) => crate::app::login::set_and_store_logged_user(user),
            DownMsg::ResgiterErrors => println!("Regist error"),
            DownMsg::GetSchool { id, name } => {
                use crate::app::school::{school, School};
                school().set(Some(School { id, name }))
            }
            DownMsg::AddedSchool(school) => local_storage()
                .insert("school", &school)
                .expect("Yüklenemedi"),
            DownMsg::Teachers(t_dmsg) => {
                match t_dmsg{
                    TeacherDownMsgs::GetTeachers(tchrs) => {
                        crate::app::school::teachers::teachers()
                            .lock_mut()
                            .replace_cloned(tchrs.clone());
                        crate::app::school::teachers::create_chunks()
                    },
                    TeacherDownMsgs::AddedTeacher(teacher) => {
                        crate::app::school::teachers::teachers()
                        .lock_mut()
                        .push_cloned(teacher);
                        crate::app::school::teachers::create_chunks()
                    },
                    TeacherDownMsgs::DeletedTeacher(id) => {
                        crate::app::school::teachers::teachers()
                            .lock_mut().retain(|t| t.id != id);
                        crate::app::school::teachers::create_chunks()
                    },
                    _ => ()
                }
            }
            DownMsg::Lectures(l_msg) => {
                match l_msg{
                    LecturesDownMsg::GetLectures(lectures) => {
                        crate::app::school::lectures::lectures()
                            .lock_mut()
                            .replace_cloned(lectures);
                        crate::app::school::lectures::create_chunks()
                    },
                    LecturesDownMsg::AddedLecture(lecture) => {
                        crate::app::school::lectures::lectures()
                            .lock_mut()
                            .push_cloned(lecture);
                        crate::app::school::lectures::create_chunks()
                    },
                    LecturesDownMsg::DeletedLecture(id) => {
                        crate::app::school::lectures::lectures()
                            .lock_mut().retain(|t| t.id != id);
                        crate::app::school::lectures::create_chunks()
                    }
                    _ => ()
                } 
            },
            DownMsg::Classes(msg) => {
                match msg {
                    ClassDownMsgs::GetClasses(classes) => crate::app::school::classes::classes()
                        .lock_mut()
                        .replace_cloned(classes.clone()),
                    ClassDownMsgs::AddedClass(class) => app::school::classes::classes()
                        .lock_mut()
                        .push_cloned(class.clone()),
                    ClassDownMsgs::DeletedClass(id) => {
                        crate::app::school::classes::classes()
                            .lock_mut().retain(|t| t.id != id);
                    }
                    _ => (),
                };
                crate::app::school::classes::create_chunks()
            },
            DownMsg::Timetables(msg) => {
                match msg {
                    TimetablesDownMsgs::GetTimetables(timetables) => {
                        crate::app::school::classes::timetables()
                            .lock_mut()
                            .replace_cloned(timetables.clone());
                        if let Some(group) = timetables.get(0) {
                            crate::app::school::classes::change_timetable(group.id.to_string());
                        }
                    },
                    TimetablesDownMsgs::AddedTimetable(timetable) => {
                        crate::app::school::classes::timetables()
                            .lock_mut()
                            .push_cloned(timetable)  
                    },
                    TimetablesDownMsgs::DeletedTimetable(id) => {
                        crate::app::school::classes::timetables()
                            .lock_mut().retain(|t| t.id != id);
                    }
                    _ => (),
                };
            },
            
            _ => (),
        }
    })
    .auth_token_getter(app::auth_token)
}

pub fn send_msg(msg: UpMsg) {
    Task::start(async {
        match connection().send_up_msg(msg).await {
            Err(_error) => {}
            Ok(_msg) => (),
        }
    });
}