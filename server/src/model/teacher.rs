use serde::*;
use crate::model::{timetable, activity};
use crate::model::class::Class;
use crate::AppState;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeacherAvailable{
    pub group_id: Option<i32>,
    pub day: timetable::Day,
    pub hours: Vec<bool>
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct TeacherAvailableForTimetables{
    user_id: i32,
    school_id: i32,
    pub(crate) day: i32,
    pub(crate) hours: Vec<bool>
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TeacherTimetable{
    pub id: i32,
    pub class_id: Vec<Class>,
    pub day_id: i32,
    pub hour: i16,
    pub subject: String,
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct Teacher{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub role_id: i32,
    pub role_name: String,
    pub is_active: bool,
    pub email: Option<String>,
    pub tel: Option<String>,
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct SimpleTeacher{
    pub id: i32
}

impl Teacher{
    pub async fn del(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<i32>{
        self.del_acts(req).await?;
        let school_id: i32 = req.param("school").expect("Okul id numarası belirtilmemiş").parse().expect("Okul numarası sayıdan oluşmalı");
        self.del_from_school(req, school_id).await?;
        let _ = sqlx::query(r#"delete from teacher_available where user_id = $1 and school_id = $2"#)
            .bind(self.id)
            .bind(school_id)
            .execute(&req.state().db_pool).await?;
        if !self.is_active{
            let _ = sqlx::query(r#"delete from users where id = $1 "#)
                .bind(self.id)
                .execute(&req.state().db_pool).await?;
        }
        Ok(self.id)
    }
    pub async fn del_act(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<i32>{
        use sqlx::prelude::PgQueryAs;
        let act_id: i32 = req.param("act_id").expect("Aktivite id numarası belirtilmemiş").parse().expect("Sayı değil");
        let act: activity::Activity  = sqlx::query_as(r#"select * from activities where id = $1 and $2 = any(classes) "#)
            .bind(act_id)
            .bind(&self.id)
            .fetch_one(&req.state().db_pool).await?;
        if act.classes.is_empty(){
            let _ = sqlx::query(r#"delete from activities where id = $1 returning *"#)
                .bind(act_id)
                .execute(&req.state().db_pool).await?;
            Ok(act_id)
        }
        else {
            let ids = &act.classes.into_iter().filter(|c| c != &self.id).collect::<Vec<i32>>();
            let _ = sqlx::query(r#"update activities set classes = $2 where id = $1"#)
                .bind(act_id)
                .bind(&ids)
                .execute(&req.state().db_pool).await?;
            Ok(act_id)
        }
    }
    pub async fn del_acts(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<&Self>{
        use sqlx::prelude::PgQueryAs;
        let acts: Vec<activity::Activity>  = sqlx::query_as(r#"select * from activities where $1 = any(teachers) or teacher = $1"#)
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        for a in acts {
            let _ = sqlx::query(r#"delete from activities where teacher = $1"#)
                .bind(a.id)
                .execute(&req.state().db_pool).await?;
            //return del
            if let Some(teachers) = a.teachers {
                if teachers.is_empty() {
                    let _ = sqlx::query(r#"delete from activities where id = $1 returning *"#)
                        .bind(a.id)
                        .execute(&req.state().db_pool).await?;
                    //return del
                } else {
                    let ids = &teachers.into_iter().filter(|t| t != &self.id).collect::<Vec<i32>>();
                    let _ = sqlx::query(r#"update from activities set teachers = $2 where id = $1"#)
                        .bind(a.id)
                        .bind(&ids)
                        .execute(&req.state().db_pool).await?;
                }
            }
        }
        Ok(self)
    }
    pub async fn del_from_school(&self, req: &tide::Request<AppState>, school_id: i32) -> sqlx_core::Result<i32> {
        let _ = sqlx::query(r#"delete from school_users where school_users.school_id = $1 and school_users.user_id = $2"#)
            .bind(&school_id)
            .bind(&self.id)
            .execute(&req.state().db_pool).await?;
        Ok(self.id)
    }
}