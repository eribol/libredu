use shared::models::class::{AddClass, Class};
use shared::msgs::classes::*;
use shared::DownMsg;
use sqlx::Row;

use super::auth::POSTGRES;
use moon::tokio_stream::StreamExt;

pub async fn add_class(id: i32, form: AddClass) -> DownMsg {
    let db = POSTGRES.write().await;
    let mut school_group =
        sqlx::query(r#"select * from class_groups where id = $1 and school = $2"#)
            .bind(&form.group_id)
            .bind(&id)
            .fetch(&*db);
    if let Some(_) = school_group.try_next().await.unwrap() {
        let mut row = sqlx::query(
            r#"insert into classes(kademe, sube, school, group_id) 
                        values($1, $2, $3, $4)
                        returning id, kademe, sube, group_id"#,
        )
        .bind(&form.kademe)
        .bind(&form.sube)
        .bind(&id)
        .bind(&form.group_id)
        .fetch(&*db);
        if let Some(class) = row.try_next().await.unwrap() {
            let class = shared::models::class::Class {
                id: class.try_get("id").unwrap(),
                kademe: class.try_get("kademe").unwrap(),
                sube: class.try_get("sube").unwrap(),
                group_id: class.try_get("group_id").unwrap(),
            };
            let c_msg = ClassDownMsgs::AddedClass(class);
            DownMsg::Classes(c_msg)
        } else {
            DownMsg::Classes(ClassDownMsgs::AddClassError("Class add failed".to_string()))
        }
    } else {
        DownMsg::Classes(ClassDownMsgs::AddClassError("Form group_id error".to_string()))
    }
}

pub async fn get_classes(id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut row = sqlx::query(
        r#"select id, kademe, sube, group_id from classes
                        where school = $1"#,
    )
    .bind(&id)
    .fetch(&*db);
    let mut classes = vec![];
    while let Some(class) = row.try_next().await.unwrap() {
        let c = Class {
            id: class.try_get("id").unwrap(),
            sube: class.try_get("sube").unwrap(),
            kademe: class.try_get("kademe").unwrap(),
            group_id: class.try_get("group_id").unwrap(),
        };
        classes.push(c);
    }
    DownMsg::Classes(ClassDownMsgs::GetClasses(classes))
}

pub async fn del_class(class_id: i32, school_id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut row = sqlx::query(
        r#"delete from classes where id = $1 and school = $2 returning id"#,
    )
    .bind(&class_id)
    .bind(&school_id)
    .fetch(&*db);
    if let Some(_) = row.try_next().await.unwrap() {
        return DownMsg::Classes(ClassDownMsgs::DeletedClass(class_id))
    }
    DownMsg::Classes(ClassDownMsgs::DeletedClass(class_id))
}

/*
pub async fn get_limitations(auth_token: Option<AuthToken>, class_id: i32) -> DownMsg {
    match auth::auth(auth_token).await {
        Some(manager) => match get_school(manager).await {
            DownMsg::GetSchool { .. } => {
                let db = POSTGRES.read().await;
                let mut row = sqlx::query(
                    r#"select class_id, day, hours from class_available
                        where class_id = $1"#,
                )
                .bind(&class_id)
                .fetch(&*db);
                let mut limitations = vec![];
                while let Some(class) = row.try_next().await.unwrap() {
                    let c = ClassLimitation {
                        class_id: class.try_get("class_id").unwrap(),
                        day: class.try_get("day").unwrap(),
                        hours: class.try_get("hours").unwrap(),
                        //group_id: class.try_get("group_id").unwrap(),
                    };
                    limitations.push(c);
                }
                DownMsg::Classes(shared::msgs::classes::ClassDownMsgs::GetLimitations(
                    limitations,
                ))
            }
            _ => DownMsg::AuthError("Not school".to_string()),
        },
        None => DownMsg::AuthError("No auth".to_string()),
    }
}

pub async fn get_activities(auth_token: Option<AuthToken>, class_id: i32) -> DownMsg {
    match auth::auth(auth_token).await {
        Some(manager) => match get_school(manager).await {
            DownMsg::GetSchool { .. } => {
                let db = POSTGRES.read().await;
                let mut row = sqlx::query(
                    r#"select * from activities
                        where $1 = any(classes)"#,
                )
                .bind(&class_id)
                .fetch(&*db);
                let mut activities = vec![];
                while let Some(act) = row.try_next().await.unwrap() {
                    let a = Activity {
                        id: act.try_get("id").unwrap(),
                        subject: act.try_get("subject").unwrap(),
                        hour: act.try_get("hour").unwrap(),
                        classes: act.try_get("classes").unwrap(),
                        teachers: act.try_get("teachers").unwrap(),
                        partner_activity: act.try_get("partner_activity").unwrap(),
                        blocks: act.try_get("blocks").unwrap(),
                    };
                    activities.push(a);
                }
                DownMsg::Classes(shared::msgs::classes::ClassDownMsgs::GetActivities(
                    activities,
                ))
            }
            _ => DownMsg::AuthError("Not school".to_string()),
        },
        None => DownMsg::AuthError("No auth".to_string()),
    }
}
*/
