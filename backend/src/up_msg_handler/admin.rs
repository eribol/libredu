use moon::{NaiveDateTime, Utc};
use shared::msgs::admin::{SchoolManager, AdminSchool, AdminUpMsgs, AdminDownMsgs};
use shared::msgs::classes::*;
use shared::{DownMsg, School};
use sqlx::Row;

use super::auth::POSTGRES;
use moon::tokio_stream::StreamExt;

pub async fn admin_msgs(msg: AdminUpMsgs)->DownMsg{
    let a_msg = get_schools().await;
    DownMsg::Admin(a_msg)
}
pub async fn get_schools() -> AdminDownMsgs {
    let db = POSTGRES.read().await;
    let mut schools =
        sqlx::query(r#"select school.id, school.name, users.id as user_id, users.first_name, users.last_name, users.last_login from school 
            inner join users on users.id = school.manager
            order by school.id desc limit 20"#)
            .fetch(&*db);
    let mut schs = vec![];
    while let Some(row) = schools.try_next().await.unwrap() {
        let s = School{
            id: row.try_get("user_id").unwrap(),
            name: row.try_get("name").unwrap(),
        };
        let u = SchoolManager{
            id: row.try_get("id").unwrap(),
            first_name: row.try_get("first_name").unwrap(),
            last_name: row.try_get("last_name").unwrap(),
            last_login: row.try_get("last_login").unwrap_or(Utc::now().naive_utc()),
        };
        let s_admin = AdminSchool{
            school: s,
            principle: u,
        };
        schs.push(s_admin);
    }
    AdminDownMsgs::LastSchools(schs)
}