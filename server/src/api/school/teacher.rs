use tide::Request;
use crate::AppState;
use crate::request::{Auth, SchoolAuth};
use http_types::{StatusCode, Method, Body};
use crate::model::{timetable};
use crate::model::activity;
use crate::model::teacher;
use crate::model::activity::Activity;
use crate::model::class::Class;
use crate::model::teacher::{TeacherTimetable, Teacher};
use serde::*;

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct SimpleAct{
    pub(crate) id: i32,
    pub(crate) teacher: Option<i32>,
    pub(crate) classes: Vec<i32>,
    //class: i32,
    pub(crate) subject: i32,
    pub(crate) hour: i16,
    split: bool
}

pub async fn get_activities(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 4 {
        let teacher = req.get_teacher().await?;
        let group = req.get_group().await?;
        //use sqlx_core::cursor::Cursor;
        //use sqlx_core::row::Row;
        let acts: Vec<Activity> = teacher.get_acts_for_group(&req, school_auth.school.id, group.id).await?;
        let mut activities: Vec<activity::FullActivity> = Vec::new();
        for a in acts {
            let subject = school_auth.school.get_subjects(&req).await?.into_iter().find(|s| s.id == a.subject).unwrap();
            //let class: class::Class = sqlx::query_as("SELECT * FROM classes WHERE id = $1").bind(&a.class).fetch_one(&req.state().db_pool).await?;
            let teachers = school_auth.school.get_teachers(&req).await?.into_iter().filter(|t| a.teachers.iter().any(|a2| a2 == &t.id)).collect::<Vec<Teacher>>();
            let classes = school_auth.school.get_classes(&req).await?.into_iter().filter(|c| a.classes.iter().any(|c2| c2 == &c.id)).collect::<Vec<Class>>();
            let act = activity::FullActivity {
                id: a.id,
                subject: subject.clone(),
                hour: a.hour,
                split: false,
                classes: classes.clone(),
                teachers
            };
            activities.push(act.clone());
        }
        res.set_body(Body::from_json(&activities)?);
        Ok(res)
    } else {
        Ok(res)
    }
}

pub async fn del_activities(req: Request<AppState>) -> tide::Result {
    //let school_id: i32 = req.param("school")?.parse()?;
    let teacher_id: i32 = req.param("teacher_id")?.parse()?;
    let act_id: i32 = req.param("act_id")?.parse()?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    let user = req.user().await?;
    if school_auth.role < 3 || user.id == teacher_id || user.is_admin {
        let mut res = tide::Response::new(StatusCode::Ok);
        //let ids = school_auth.school.get_classes_ids(&req).await?;
        let teacher = req.get_teacher().await?;

        teacher.del_act(&req).await?;
        res.set_body(Body::from_json(&act_id)?);

        Ok(res)
    }
    else {
        let res = tide::Response::new(StatusCode::Unauthorized);
        Ok(res)
    }
}

pub async fn teacher_detail(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 8 {
        let teacher = req.get_teacher().await;
        match teacher{
            Ok(t) =>{
                res.set_body(Body::from_json(&t)?);
                Ok(res)
            }
            Err(_) => {
                Ok(res)
            }
        }
    } else {
        Ok(res)
    }
}

pub async fn limitations(mut req: Request<AppState>) -> tide::Result {
    let teacher_id: i32 = req.param("teacher_id")?.parse()?;
    let user = req.user().await.unwrap();
    let post = req.body_json::<Vec<teacher::TeacherAvailable>>().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 2 || user.id == teacher_id {
        let mut res = tide::Response::new(StatusCode::Ok);
        let teacher = school_auth.school.get_teacher(&req, teacher_id).await?;
        teacher.limitations(&req, school_auth.school.id, Some(post)).await?;
        res.set_body(Body::from_json(&teacher_id)?);
        Ok(res)
    } else {
        let res = tide::Response::new(StatusCode::Unauthorized);
        Ok(res)
    }
}

pub async fn get_limitations(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let teacher_id: i32 = req.param("teacher_id")?.parse()?;
    let group_id: i32 = req.param("group_id")?.parse()?;
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    let user = req.user().await.unwrap();
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 8 || user.id == teacher_id {
        let mut teacher_availables = sqlx::query(r#"SELECT
                        days.id, days.name, teacher_available.hours, teacher_available.user_id, teacher_available.school_id, teacher_available.group_id
                        FROM teacher_available inner join days on teacher_available.day = days.id
                        WHERE teacher_available.user_id = $1 and teacher_available.school_id = $2 and teacher_available.group_id = $3"#)
            .bind(&teacher_id)
            .bind(&school_auth.school.id)
            .bind(&group_id)
            .fetch(&req.state().db_pool);
        let mut availables: Vec<teacher::TeacherAvailable> = Vec::new();
        while let Some(row) = teacher_availables.next().await? {
            let available = teacher::TeacherAvailable {
                group_id: row.get(3),
                day: timetable::Day {
                    id: row.get(0),
                    name: row.get(1)
                },
                hours: row.get(2)
            };
            availables.push(available);
        }
        availables.sort_by(|a, b| b.day.id.cmp(&a.day.id));
        res.set_body(Body::from_json(&availables)?);
        Ok(res)
    } else {
        Ok(res)
    }
}

pub async fn timetables(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    //let school_id: i32 = req.param("school")?.parse()?;
    let teacher_id: i32 = req.param("teacher_id")?.parse()?;
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 2 {
        match req.method() {
            Method::Get => {
                let group = req.get_group().await?;
                let mut class = sqlx::query("SELECT class_timetable.id, classes.id, classes.kademe, classes.sube, classes.school, classes.group_id,
                            class_timetable.day_id, class_timetable.hour, subjects.name, activities.teachers
                            FROM class_timetable inner join activities on class_timetable.activities = activities.id
                            inner join subjects on activities.subject = subjects.id
                            inner join classes on class_timetable.class_id = classes.id
                            WHERE $1 = any(activities.teachers) and classes.group_id = $2")
                    .bind(&teacher_id)
                    .bind(&group.id)
                    .fetch(&req.state().db_pool);
                let mut teacher_timetables: Vec<TeacherTimetable> = Vec::new();
                while let Some(row) = class.next().await? {
                    let teacher_timetable = TeacherTimetable {
                        id: row.get(0),
                        class_id: vec![Class {
                            id: row.get(1),
                            kademe: row.get(2),
                            sube: row.get(3),
                            school: row.get(4),
                            group_id: row.get(5)
                        }],
                        day_id: row.get(6),
                        hour: row.get(7),
                        subject: row.get(8)
                    };
                    teacher_timetables.push(teacher_timetable);
                }
                res.set_body(Body::from_json(&teacher_timetables)?);
                Ok(res)
            }
            _ => {
                Ok(res)
            }
        }
    } else {
        Ok(res)
    }
}

pub async fn del_teacher(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    //use sqlx_core::postgres::PgQueryAs;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 2 {
        //use crate::model::teacher;
        let tchr = req.get_teacher().await;
        match tchr{
            Ok(teacher) => {
                teacher.del(&req).await?;
                res.set_body(Body::from_json(&teacher.id)?);
                Ok(res)
            }
            Err(_) => {
                res.set_body(Body::from_json(&0)?);
                Ok(res)
            }
        }
    }
    else {
        Ok(res)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct UpdateTeacherForm{
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub email: String,
    pub tel: String,
    password1: String,
    password2: String,
}
pub async fn patch_teacher(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let teacher_id: i32 = req.param("teacher_id")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 2 {
        let teacher = req.get_teacher().await;
        match teacher {
            Ok(_t) => {
                use crate::model::teacher::SimpleTeacher;
                let form: UpdateTeacherForm = req.body_json().await?;
                let update_teacher: SimpleTeacher = sqlx::query_as(r#"UPDATE users
                            set is_active = true, email = $1, tel = $2, key = $3, password = $4, first_name = $5, last_name = $6
                            WHERE id=$7 returning id"#)
                    .bind(&form.email)
                    .bind(&bcrypt::hash(form.tel, 8).unwrap())
                    .bind(&uuid::Uuid::new_v4().to_string())
                    .bind(&bcrypt::hash(&form.password1, 10).unwrap())
                    .bind(&form.first_name)
                    .bind(&form.last_name)
                    .bind(&teacher_id)
                    .fetch_one(&req.state().db_pool).await?;
                res.set_body(Body::from_json(&update_teacher)?);
                Ok(res)
            }
            Err(_) => {
                //res.set_body(Body::from_json(&teacher)?);
                Ok(res)
            }
        }
    } else {
        Ok(res)
    }
}