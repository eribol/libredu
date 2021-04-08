use chrono::NaiveTime;
use serde::*;
use crate::AppState;
use crate::model::{teacher, timetable, class, activity};
use crate::request::Auth;
use crate::model::class::Class;
//use tide::Error;


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

impl ClassGroups {
    pub async fn get_classes(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<class::ClassForTimetables>> {
        use sqlx::prelude::PgQueryAs;
        let classes: Vec<class::ClassForTimetables> = sqlx::query_as("SELECT * FROM classes WHERE school = $1 and group_id = $2 order by kademe, sube")
            .bind(&self.school)
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(classes)
    }
    pub async fn get_classes_ids(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<i32>> {
        use sqlx::prelude::PgQueryAs;
        let ids: (Option<Vec<i32>>, ) = sqlx::query_as(r#"select array_agg(id) from  classes where school= $1 and group_id = $2"#)
            .bind(&self.school)
            .bind(&self.id)
            .fetch_one(&req.state().db_pool).await?;
        match ids.0 {
            Some(i) => {
                Ok(i)
            }
            None => {
                Ok(vec![])
            }
        }
    }
    pub async fn get_timetables(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<timetable::NewTimetable>> {
        use sqlx::prelude::PgQueryAs;
        let timetables: Vec<timetable::NewTimetable> = sqlx::query_as("SELECT class_timetable.class_id, class_timetable.day_id, class_timetable.hour, class_timetable.activities
                            FROM class_timetable inner join classes on class_timetable.class_id = classes.id
                            WHERE class_timetable.class_id = any($1) and classes.group_id = $2")
            .bind(&self.get_classes_ids(&req).await?)
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(timetables)
    }
    pub async fn get_tat(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<teacher::TeacherAvailableForTimetables>> {
        use sqlx::prelude::PgQueryAs;
        let tat: Vec<teacher::TeacherAvailableForTimetables> = sqlx::query_as(r#"select * from teacher_available where school_id = $1 and group_id = $2"#)
            .bind(&self.school)
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(tat)
    }
    pub async fn get_cat(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<class::ClassAvailableForTimetables>> {
        use sqlx::prelude::PgQueryAs;
        let cat: Vec<class::ClassAvailableForTimetables> = sqlx::query_as(r#"select * from class_available inner join classes on class_available.class_id = classes.id
                        where classes.school = $1 and classes.group_id = $2"#)
            .bind(&self.school)
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(cat)
    }
    pub async fn get_acts(&self, req: &tide::Request<AppState>) -> tide::Result<Vec<timetable::Activity>> {
        use sqlx::prelude::PgQueryAs;
        let acts: Vec<timetable::Activity> = sqlx::query_as(r#"select activities.id, activities.subject, activities.hour, activities.teacher, activities.split, activities.classes
                        from activities where classes && $1::integer[]"#)
            //.bind(&school_id)
            .bind(&self.get_classes_ids(&req).await?)
            .fetch_all(&req.state().db_pool).await?;
        Ok(acts)
    }
    pub async fn add_acts(&self, req: &mut tide::Request<AppState>) -> tide::Result<Vec<activity::FullActivity>> {
        use sqlx::prelude::PgQueryAs;
        let mut act: activity::NewActivity = req.body_json().await?;
        act.classes.sort();
        act.classes.dedup();
        //act.classes.retain(|c| *c & 1 == 1);
        //act.teachers.retain(|t| *t & 1 == 1);
        let school = req.get_school().await.unwrap();
        let teacher = school.get_teachers(&req).await?.into_iter().find(|t| t.id == act.teacher).unwrap();
        let subject = school.get_subjects(&req).await?.into_iter().find(|s| s.id == act.subject).unwrap();
        let classes = school.get_classes(&req).await?.into_iter().filter(|c| act.classes.iter().any(|c2| c2 == &c.id)).collect::<Vec<Class>>();
        let mut acts: Vec<activity::FullActivity> = vec![];
        for h in act.hour.split(" ").collect::<Vec<&str>>() {
            match h.parse::<i16>() {
                Ok(hour) => {
                    let insert: activity::Activity = sqlx::query_as("insert into activities(teacher, subject, hour, split, classes) values($1, $2, $3, $4, $5) \
                                            returning id, subject, teacher, hour, split, classes, teachers")
                        //.bind(&act.class)
                        .bind(&act.teacher)
                        .bind(&act.subject)
                        .bind(&hour)
                        .bind(&act.split)
                        .bind(&act.classes)
                        .fetch_one(&req.state().db_pool).await?;

                    let new_act = activity::FullActivity {
                        id: insert.id,
                        subject: subject.clone(),
                        teacher: teacher.clone(),
                        hour,
                        split: false,
                        classes: classes.clone(),
                        teachers: Some(vec![])
                    };
                    acts.push(new_act)
                }
                Err(_) => {
                    //Error
                }
            }
        }
        Ok(acts)
    }
}

