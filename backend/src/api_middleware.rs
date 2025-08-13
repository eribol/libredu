use crate::{
    connection::{
        admin::{get_activities, get_timetables},
        school::get_school_api,
    },
    up_msg_handler::{
        classes::get_classes, lectures::get_lectures, teachers::get_teachers,
        timetables::get_class_groups,
    },
    MyUp,
};
use moon::{
    actix_web::{error, Responder},
    serde_json,
};
use shared::DownMsg;

pub async fn school_api(msg: &MyUp) -> impl Responder {
    let api_key = get_school_api(msg.id).await;
    println!("{}", msg.api_key);
    if api_key != msg.api_key {
        return Err(error::ErrorUnauthorized("Api Key uyuşmazlığı"));
    } else {
        match msg.msg {
            crate::MyMsg::GetTeachers => {
                let teachers = get_teachers(msg.id).await;
                match teachers {
                    DownMsg::Teachers(t) => {
                        let teachers = serde_json::to_string(&t).unwrap();
                        // println!("{teachers:?}");
                        return Ok(teachers);
                    }
                    _ => {
                        return Ok("a".to_string());
                    }
                }
            }
            crate::MyMsg::GetTimetables => {
                let timetables = get_class_groups(msg.id).await;
                match timetables {
                    DownMsg::Timetables(tt) => {
                        let t = serde_json::to_string(&tt).unwrap();
                        return Ok(t);
                    }
                    _ => {
                        return Ok("a".to_string());
                    }
                }
            }
            crate::MyMsg::GetClasses => {
                let class = get_classes(msg.id).await;
                match class {
                    DownMsg::Classes(cc) => {
                        let c = serde_json::to_string(&cc).unwrap();
                        return Ok(c);
                    }
                    _ => {
                        return Ok("a".to_string());
                    }
                }
            }
            crate::MyMsg::GetLectures => {
                let class = get_lectures(msg.id).await;
                match class {
                    DownMsg::Lectures(lecs) => {
                        // println!("{cc:?}");c
                        let c = serde_json::to_string(&lecs).unwrap();
                        return Ok(c);
                    }
                    _ => {
                        return Ok("a".to_string());
                    }
                }
            }
            crate::MyMsg::GetActivities(id) => {
                let tt = get_class_groups(msg.id).await;
                if let DownMsg::Timetables(
                    shared::msgs::timetables::TimetablesDownMsgs::GetTimetables(tt),
                ) = tt
                {
                    if tt.iter().find(|t| t.id == id).is_some() {
                        let acts = get_activities(id).await;
                        println!("{acts:?}");
                        let c = serde_json::to_string(&acts).unwrap();
                        return Ok(c);
                    }
                    return Ok("a".to_string());
                }
                return Ok("a".to_string());
            }
            _ => {
                return Ok("b".to_string());
            }
        }
        // return Ok("oldu");
    }
}
