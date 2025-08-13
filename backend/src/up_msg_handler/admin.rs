use moon::AuthToken;
use shared::msgs::admin::{AdminDownMsgs, AdminUpMsgs};
use shared::DownMsg;

use crate::connection::admin::get_schools;
use crate::connection::get_user;
use crate::user::get_user_with_id;

pub async fn admin_msgs(msg: AdminUpMsgs, auth_token: Option<AuthToken>) -> DownMsg {
    let mut d_msg = DownMsg::AuthError("Not auth".to_string());
    if let Some(auth) = auth_token {
        let user = get_user(auth.as_str()).await;
        let u = get_user_with_id(user.unwrap()).await;
        if let Ok(user) = u {
            if user.is_admin {
                d_msg = DownMsg::Admin(run_msg(msg).await)
            }
        }
    }
    d_msg
}

async fn run_msg(msg: AdminUpMsgs) -> AdminDownMsgs {
    match msg {
        AdminUpMsgs::GetLastSchools => get_schools().await,
        AdminUpMsgs::SearchSchool(group_id) => {
            crate::connection::admin::get_classes(group_id).await
        }
        AdminUpMsgs::GetTeachers(school_id) => {
            crate::connection::admin::get_teachers(school_id).await
        }
        AdminUpMsgs::GetClasses(group_id) => crate::connection::admin::get_classes(group_id).await,
        AdminUpMsgs::GetTimetables(group_id) => {
            crate::connection::admin::get_timetables(group_id).await
        }
        AdminUpMsgs::GetActivities(group_id) => {
            crate::connection::admin::get_activities(group_id).await
        }
        AdminUpMsgs::GetClassesLimitations(group_id) => {
            crate::connection::admin::get_classes_limitations(group_id).await
        }
        AdminUpMsgs::GetTeachersLimitations(group_id) => {
            crate::connection::admin::get_teachers_limitations(group_id).await
        }
        AdminUpMsgs::UpdateClassLimitations(lims) => {
            crate::connection::admin::update_class_limitations(lims[0].clone()).await
        }
        AdminUpMsgs::UpdateTeacherLimitations(lims) => {
            crate::connection::admin::update_teacher_limitations(lims).await
        }
        AdminUpMsgs::DelAct(act_id) => crate::connection::admin::del_act(act_id).await,
        AdminUpMsgs::GetSchoolMessages(id) => {
            super::connection::admin::get_school_messages(id).await
        }
        AdminUpMsgs::SendMessage(msg) => super::connection::admin::new_message(msg).await,
    }
}
