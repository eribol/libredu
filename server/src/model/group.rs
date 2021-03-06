use chrono::NaiveTime;
use serde::*;
use crate::AppState;
use crate::model::{teacher, timetable, class};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClassGroups{
    pub id: i32,
    pub name: String,
    pub hour: i32,
    pub school: i32
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Schedules{
    group_id: i32,
    hour: i32,
    start_time: NaiveTime,
    end_time: NaiveTime
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct AddGroup{
    name: String,
    hour: i32
}
pub async fn get_group(school_id: i32, group_id: i32, req: &tide::Request<AppState>) -> sqlx_core::Result<ClassGroups>{
    use sqlx::prelude::PgQueryAs;
    let group: ClassGroups = sqlx::query_as("SELECT * FROM class_groups WHERE school = $1 and id = $2")
        .bind(&school_id)
        .bind(&group_id)
        .fetch_one(&req.state().db_pool).await?;
    Ok(group)
}

impl ClassGroups{
    pub async fn get_classes(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<class::ClassForTimetables>>{
        use sqlx::prelude::PgQueryAs;
        let classes: Vec<class::ClassForTimetables> = sqlx::query_as("SELECT * FROM classes WHERE school = $1 and group_id = $2 order by kademe, sube")
            .bind(&self.school)
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(classes)
    }

    pub async fn get_classes_ids(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<i32>>{
        use sqlx::prelude::PgQueryAs;
        let ids: (Option<Vec<i32>>, ) = sqlx::query_as(r#"select array_agg(id) from  classes where school= $1 and group_id = $2"#)
            .bind(&self.school)
            .bind(&self.id)
            .fetch_one(&req.state().db_pool).await?;
        match ids.0{
            Some(i) => {
                Ok(i)
            }
            None =>{
                Ok(vec![])
            }
        }
    }
    pub async fn get_timetables(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<timetable::NewTimetable>>{
        use sqlx::prelude::PgQueryAs;
        let timetables: Vec<timetable::NewTimetable> = sqlx::query_as("SELECT class_timetable.class_id, class_timetable.day_id, class_timetable.hour, class_timetable.activities
                            FROM class_timetable inner join classes on class_timetable.class_id = classes.id
                            WHERE class_timetable.class_id = any($1) and classes.group_id = $2")
            .bind(&self.get_classes_ids(&req).await?)
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(timetables)
    }
    pub async fn get_tat(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<teacher::TeacherAvailableForTimetables>>{
        use sqlx::prelude::PgQueryAs;
        let tat: Vec<teacher::TeacherAvailableForTimetables> = sqlx::query_as(r#"select * from teacher_available where school_id = $1 and group_id = $2"#)
            .bind(&self.school)
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(tat)
    }
    pub async fn get_cat(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<class::ClassAvailableForTimetables>>{
        use sqlx::prelude::PgQueryAs;
        let cat: Vec<class::ClassAvailableForTimetables> = sqlx::query_as(r#"select * from class_available inner join classes on class_available.class_id = classes.id
                        where classes.school = $1 and classes.group_id = $2"#)
            .bind(&self.school)
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(cat)
    }

}

