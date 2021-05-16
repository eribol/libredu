use serde::*;
use crate::model::subject::Subject;
use crate::model::teacher::Teacher;
use crate::AppState;
use crate::model::activity;

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct Class{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub school: i32,
    pub group_id: i32
}
#[derive(Deserialize, Serialize, sqlx::FromRow, Debug, PartialEq, Clone)]
pub struct ClassForTimetables{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub school: i32,
    pub teacher: Option<i32>,
}

impl From<Class> for ClassForTimetables{
    fn from(item: Class) -> Self {
        ClassForTimetables{
            id: item.id,
            kademe: item.kademe,
            sube: item.sube,
            school: item.school,
            teacher: None
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct ClassTimetable{
    pub id: i32,
    pub class_id: i32,
    pub day_id: i32,
    pub hour: i16,
    pub subject: String,
    pub activity: ClassTimetableActivity
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct ClassTimetableActivity{
    pub id: i32,
    pub teacher: Teacher
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct ClassActivity{
    pub id: i32,
    pub subject: Subject,
    pub teacher: Teacher,
    pub hour: i16,
    pub split: bool
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NewClass{
    pub kademe: String,
    pub sube: String,
    pub group_id: i32
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateClass{
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub group_id: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClassAvailableForTimetables{
    pub(crate) class_id: i32,
    pub(crate) day: i32,
    pub(crate) hours: Vec<bool>
}

impl Class{
    pub async fn del(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<i32>{
        use sqlx::prelude::PgQueryAs;
        self.del_acts(req).await?;
        let _ = sqlx::query(r#"delete from classes where id = $1 "#)
            .bind(self.id)
            .execute(&req.state().db_pool).await?;
        Ok(self.id)
    }
    pub async fn del_act(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<i32>{
        use sqlx::prelude::PgQueryAs;
        let act_id: i32 = req.param("act_id").expect("Aktivite id numarası belirtilmemiş").parse().expect("Sayı değil");
        let act: activity::Activity  = sqlx::query_as(r#"select * from activities where id = $1 and $2 = any(classes) "#)
            .bind(act_id)
            .bind(&self.id)
            .fetch_one(&req.state().db_pool).await?;
        if act.classes.len() <= 1{
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
        let acts: Vec<activity::Activity>  = sqlx::query_as(r#"select * from activities where $1 = any(classes)"#)
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
         for a in acts{
            if a.classes.len() <= 1{
                let _ = sqlx::query(r#"delete from activities where id = $1 returning *"#)
                    .bind(a.id)
                    .execute(&req.state().db_pool).await?;
                //return del
            }
            else {
                let ids = &a.classes.into_iter().filter(|c| c != &self.id).collect::<Vec<i32>>();
                let _ = sqlx::query(r#"update activities set classes = $2 where id = $1"#)
                    .bind(a.id)
                    .bind(&ids)
                    .execute(&req.state().db_pool).await?;
                //return update
            }
        }
        Ok(self)
    }
}