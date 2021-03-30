use tide::Request;
use crate::AppState;
use crate::request::{Auth, SchoolAuth};
use http_types::{StatusCode, Method, Body};
use crate::model::{timetable, subject};
use crate::model::user;
use crate::model::school;
use crate::model::class;
use crate::model::activity;
use crate::model::teacher;
use crate::model::activity::{NewActivity, Activity};
use crate::model::class::Class;
use crate::model::teacher::TeacherTimetable;
use serde::*;
use crate::model::school::SchoolDetail;
use crate::model::city::{City, Town};
use crate::model::user::{SimpleTeacher};

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct Teacher{
    id: i32,
    first_name: String,
    last_name: String,
    is_active: bool,
    email: Option<String>,
    tel: Option<String>
}

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
    //let school_id: i32 = req.param("school")?.parse()?;
    let teacher_id: i32 = req.param("teacher_id")?.parse()?;
    let group_id: i32 = req.param("group_id")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 4 {
        //use sqlx_core::cursor::Cursor;
        //use sqlx_core::row::Row;
        let acts: Vec<Activity> = sqlx::query_as(r#"SELECT * FROM activities inner join subjects on activities.subject = subjects.id
                        WHERE activities.teacher = $1 order by activities.subject, activities.classes"#)
            .bind(&teacher_id)
            //.bind(&school_auth.school.id)
            .fetch_all(&req.state().db_pool).await?;
        let mut activities: Vec<activity::FullActivity> = Vec::new();
        for a in acts {
            let subject = school_auth.school.get_subjects(&req).await?.into_iter().find(|s| s.id == a.subject).unwrap();
            //let class: class::Class = sqlx::query_as("SELECT * FROM classes WHERE id = $1").bind(&a.class).fetch_one(&req.state().db_pool).await?;
            let teacher = school_auth.school.get_teachers(&req).await?.into_iter().find(|t| t.id == a.teacher).unwrap();
            let classes = school_auth.school.get_classes(&req).await?.into_iter().filter(|c| a.classes.iter().any(|c2| c2 == &c.id)).collect::<Vec<Class>>();
            let act = activity::FullActivity {
                id: a.id,
                subject: subject.clone(),
                teacher: teacher.clone(),
                hour: a.hour,
                split: false,
                classes: classes.clone(),
                teachers: None
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
    let school_id: i32 = req.param("school")?.parse()?;
    let teacher_id: i32 = req.param("teacher_id")?.parse()?;
    let act_id: i32 = req.param("act_id")?.parse()?;
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    let mut school = SchoolDetail::default();
    let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, city.pk, city.name, town.pk, town.name \
            FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk WHERE school.id = $1")
        .bind(&school_id)
        .fetch(&req.state().db_pool);
    while let Some(row) = query.next().await?{
        school = SchoolDetail{
            id: row.get(0),
            name: row.get(1),
            manager: row.get(2),
            school_type: row.get(3),
            tel: None,
            location: None,
            city: City{ pk: row.get(4), name: row.get(5) },
            town: Town{
                city: row.get(4),
                pk: row.get(6),
                name: row.get(7)
            }
        }
    }
    match req.user().await {
        Some(user)=>{
            if school.manager == user.id{
                let mut res = tide::Response::new(StatusCode::Ok);
                let _update = sqlx::query(r#"delete from activities where teacher = $1 and id = $2"#)
                    .bind(&teacher_id)
                    .bind(&act_id)
                    .execute(&req.state().db_pool).await?;
                res.set_body(Body::from_json(&act_id)?);
                Ok(res)
            }
            else{
                let res = tide::Response::new(StatusCode::Unauthorized);
                Ok(res)
            }
        }
        None=>{
            let res = tide::Response::new(StatusCode::Unauthorized);
            Ok(res)
        }
    }
}

pub async fn teacher_detail(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let teacher_id: i32 = req.param("teacher_id")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 8 {
        let teacher: Teacher = sqlx::query_as(r#"SELECT * FROM users WHERE id=$1"#)
            .bind(&teacher_id)
            .fetch_one(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&teacher)?);
        Ok(res)
    } else {
        Ok(res)
    }
}

pub async fn limitations(mut req: Request<AppState>) -> tide::Result {
    let res = tide::Response::new(StatusCode::Ok);
    let teacher_id: i32 = req.param("teacher_id")?.parse()?;
    let group_id: i32 = req.param("group_id")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    let user = req.user().await.unwrap();
    let post = req.body_json::<Vec<teacher::TeacherAvailable>>().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 2 || user.id == teacher_id {
        let find: sqlx::Result<school::SchoolTeacher> = sqlx::query_as(r#"select * from school_users where school_id= $1 and user_id = $2"#)
            .bind(&school_auth.school.id)
            .bind(&teacher_id)
            //.bind(&group_id)
            .fetch_one(&req.state().db_pool).await;
        match find {
            Ok(_) => {
                for available in post {
                    let update: sqlx::Result<school::SchoolTeacher> = sqlx::query_as(r#"update teacher_available set hours = $4 where user_id= $1 and school_id= $2 and day= $3 and group_id = $5 returning school_id, user_id, group_id"#)
                        .bind(&teacher_id)
                        .bind(&school_auth.school.id)
                        .bind(&available.day.id)
                        .bind(&available.hours)
                        .bind(&group_id)
                        .fetch_one(&req.state().db_pool).await;
                    match update {
                        Ok(_s) => {}
                        Err(_) => {
                            let _update: school::SchoolTeacher = sqlx::query_as(r#"insert into teacher_available(user_id, school_id, day, hours, group_id)
                                                        values($1, $2, $3, $4, $5) returning school_id, user_id"#)
                                .bind(&teacher_id)
                                .bind(&school_auth.school.id)
                                .bind(&available.day.id)
                                .bind(&available.hours)
                                .bind(&group_id)
                                .fetch_one(&req.state().db_pool).await?;
                        }
                    }
                }
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
                let mut class = sqlx::query("SELECT class_timetable.id, classes.id, classes.kademe, classes.sube, classes.school, classes.group_id,
                            class_timetable.day_id, class_timetable.hour, subjects.name
                            FROM class_timetable inner join activities on class_timetable.activities = activities.id
                            inner join users on activities.teacher = users.id
                            inner join subjects on activities.subject = subjects.id
                            inner join classes on class_timetable.class_id = classes.id
                            WHERE activities.teacher = $1")
                    .bind(&teacher_id)
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
    //let school_id: i32 = req.param("school")?.parse()?;
    let teacher_id: i32 = req.param("teacher_id")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role <= 2 {
        let teacher: SimpleTeacher = sqlx::query_as(r#"SELECT users.id, users.first_name, users.last_name, users.is_active, users.email, users.tel
                        FROM users inner join school_users on users.id = school_users.user_id
                        WHERE school_users.school_id = $1 and school_users.user_id = $2 and school_users.role > 1"#)
            .bind(&school_auth.school.id)
            .bind(&teacher_id)
            .fetch_one(&req.state().db_pool).await?;

        let _ = sqlx::query(r#"update classes set teacher = null WHERE teacher = $1"#)
            .bind(&teacher.id)
            .execute(&req.state().db_pool).await?;
        let _del_school_user = sqlx::query(r#"delete from teacher_available WHERE user_id= $1 and school_id = $2"#)
            .bind(&teacher.id)
            .bind(&school_auth.school.id)
            .execute(&req.state().db_pool).await?;
        if !teacher.is_active {
            let _ = sqlx::query(r#"delete from activities WHERE teacher=$1"#)
                .bind(&teacher.id)
                .execute(&req.state().db_pool).await?;
            let _ = sqlx::query(r#"delete from school_users WHERE user_id = $1 and school_id = $2"#)
                .bind(&teacher_id)
                .bind(&school_auth.school.id)
                .execute(&req.state().db_pool).await?;
            let _ = sqlx::query(r#"delete from users WHERE id = $1"#)
                .bind(&teacher.id)
                .execute(&req.state().db_pool).await?;
            res.set_body(Body::from_json(&teacher_id)?);
            Ok(res)
        } else {
            let _ = sqlx::query(r#"delete from school_users WHERE user_id = $1 and school_id = $2"#)
                .bind(&teacher_id)
                .bind(&school_auth.school.id)
                .execute(&req.state().db_pool).await?;
            let classes: (Option<Vec<i32>>, ) = sqlx::query_as(r#"select array_agg(id) from  classes where school= $1"#)
                .bind(&school_auth.school.id)
                .fetch_one(&req.state().db_pool).await?;
            let _ = sqlx::query(r#"delete from activities WHERE teacher=$1 and array[$2::int[]] @> classes "#)
                .bind(&teacher_id)
                .bind(classes.0.unwrap_or_default())
                .execute(&req.state().db_pool).await?;
            res.set_body(Body::from_json(&teacher_id)?);
            Ok(res)
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
        let teacher: sqlx::Result<Teacher> = sqlx::query_as(r#"SELECT * FROM users
                        WHERE id=$1 and is_active = false and email is null and key is null and tel is null"#)
            .bind(&teacher_id)
            .fetch_one(&req.state().db_pool).await;
        match teacher {
            Ok(_t) => {
                let form: UpdateTeacherForm = req.body_json().await?;
                let update_teacher: Teacher = sqlx::query_as(r#"UPDATE users
                            set is_active = true, email = $1, tel = $2, key = $3, password = $4, first_name = $5, last_name = $6
                            WHERE id=$7 returning id, first_name, last_name, is_active, email, tel"#)
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