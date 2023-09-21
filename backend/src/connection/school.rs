use moon::*;
use shared::{models::{school::FullSchool, timetables::AddTimetable}, DownMsg};
use sqlx::FromRow;

use super::sql::POSTGRES;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(crate = "serde")]
pub struct School {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(crate = "serde")]
pub struct UpdateSchool {
    name: Option<String>,
    manager: Option<i32>,
    tel: Option<String>,
    location: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(crate = "serde")]
pub struct Timetable {
    pub id: i32,
    pub name: String,
    hour: i32,
}
pub async fn add_school(auth_token: Option<AuthToken>, name: String) -> DownMsg {
    match auth(auth_token.clone()).await {
        Ok(manager) => {
            let school: sqlx::Result<School> = sqlx::query_as(
                "insert into school(name, manager) values($1, $2) returning id, name",
            )
            .bind(&name)
            .bind(manager)
            .fetch_one(&*POSTGRES.read().await)
            .await;
            match school {
                Ok(school) => {
                    let tt = AddTimetable{
                        name: "Default".to_string(),
                        hour: 8
                    };
                    crate::up_msg_handler::timetables::add_timetable(
                        tt,
                        school.id
                    ).await;
                    DownMsg::GetSchool {
                        id: school.id,
                        name: school.name,
                    }
                }
                Err(e) => DownMsg::AddSchoolError(e.to_string()),
            }
        }
        Err(e) => DownMsg::AddSchoolError(e.to_owned()),
    }
}

pub async fn update_school(auth_token: Option<AuthToken>, form: &FullSchool) -> DownMsg {
    match auth(auth_token.clone()).await {
        Ok(token) => {
            println!("{:?}", form.clone());
            let school: sqlx::Result<School> = sqlx::query_as(
                "update school inner join users on users.id = school_users.user_id 
                set school.name = $2, school.manager = $3, school.tel = $4
                school_users.role = 1  
                where school.id = $1 and school.manager = $5 school_users.school_id = $1 and school_users.user_id = $3 and users.id = $3 and users.is_active and users.email is not null",
            )
            .bind(&form.id)
            .bind(&form.name)
            .bind(form.manager)
            .bind(&form.phone)
            .bind(token)
            //.bind(&form.location)
            .fetch_one(&*POSTGRES.read().await).await;
            match school {
                Ok(school) => {
                    let _ = sqlx::query(
                        "update school_users set role = 1 inner join users on users.id = school_users.user_id 
                        where school_users.school_id = $1 and school_users.user_id = $2 and users.is_active and users.email is not null",
                    )
                    .bind(&form.id)
                    .bind(&token)
                    .execute(&*POSTGRES.read().await).await;
                    DownMsg::AddedSchool(shared::School {
                        id: school.id,
                        name: school.name,
                    })
                }
                Err(e) => DownMsg::AddSchoolError(e.to_string()),
            }
        }
        Err(e) => DownMsg::AddSchoolError(e.to_string()),
    }
}

pub async fn get_school(manager: i32) -> sqlx::Result<School> {
    let db = POSTGRES.read().await;
    let school: sqlx::Result<School> =
        sqlx::query_as(r#"select id, name from school where manager = $1"#)
            .bind(manager)
            .fetch_one(&*db)
            .await;
    school
}

pub async fn auth(auth_token: Option<AuthToken>) -> Result<i32, String> {
    match auth_token {
        Some(auth) => {
            use crate::connection::get_user;
            let user_id: redis::RedisResult<i32> = get_user(&auth.into_string()).await;
            match user_id {
                Ok(id) => Ok(id),
                Err(e) => Err(e.to_string()),
            }
        }
        None => Err("Not auth".to_string()),
    }
}
