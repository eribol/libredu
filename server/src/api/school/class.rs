use tide::Request;
use crate::AppState;

use http_types::{StatusCode, Method, Body};
use crate::model::class::{ClassTimetable, ClassTimetableActivity, Class, ClassActivity, UpdateClass};
use crate::model::school::SchoolDetail;
use crate::request::{Auth, SchoolAuth};
use crate::model::timetable::{ClassAvailable, InsertClassAvailable};
use crate::model::timetable::Day;
use crate::model::activity::{Activity};
use crate::model::city::{City, Town};
use crate::model::student::SimpleStudent;
use crate::model::subject::Subject;
use crate::model::teacher::Teacher;

pub async fn activities(req: Request<AppState>) -> tide::Result {
    let class_id: i32 = req.param("class_id")?.parse()?;
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    //let school_auth: &SchoolAuth = req.ext().unwrap();
    let mut res = tide::Response::new(StatusCode::Ok);
    let mut cursor = sqlx::query(r#"SELECT
                        activities.id, subjects.id, subjects.name, users.id, users.first_name, users.last_name, activities.hour, activities.split, subjects.kademe,subjects.optional, users.is_active
                        FROM activities inner join users on activities.teacher= users.id inner join subjects on activities.subject = subjects.id
                        WHERE $1 = any(activities.classes) order by activities.subject, activities.teacher"#)
        .bind(&class_id)
        .fetch(&req.state().db_pool);
    let mut acts: Vec<ClassActivity> = Vec::new();

    while let Some(row) = cursor.next().await? {
        //println!("{:?}", format!("{}",row.to_string()));
        let act = ClassActivity {
            id: row.get(0),
            subject: Subject { name: row.get(2), id: row.get(1), kademe: row.get(8), optional: row.get(9), school: 0 },
            teacher: Teacher {
                id: row.get(3),
                first_name: row.get(4),
                last_name: row.get(5),
                role_id: 0,
                role_name: "".to_string(),
                is_active: row.get(10)
            },
            hour: row.get(6),
            split: row.get(7)
        };
        acts.push(act);
    }
    res.set_body(Body::from_json(&acts)?);
    Ok(res)
}

pub async fn class_detail(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let class_id = req.param("class_id")?.parse()?;
    let group_id = req.param("group_id")?.parse()?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    let class = school_auth.school.get_class(&req, group_id, class_id).await?;
    res.set_body(Body::from_json(&class)?);
    Ok(res)
}

pub async fn class_delete(req: Request<AppState>) -> tide::Result {
    let class_id = req.param("class_id")?.parse::<i32>()?;
    let group_id = req.param("group_id")?.parse::<i32>()?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 4 {
        let mut res = tide::Response::new(StatusCode::Ok);
        let class = school_auth.school.get_class(&req, group_id, class_id).await?;
        class.del(&req).await?;
        res.set_body(Body::from_json(&class_id)?);
        Ok(res)
    } else {
        let res = tide::Response::new(StatusCode::Unauthorized);
        Ok(res)
    }
}

pub async fn del_act(req: Request<AppState>) -> tide::Result{
    let act_id: i32 = req.param("act_id")?.parse()?;
    let class_id: i32 = req.param("class_id")?.parse()?;
    let group_id: i32 = req.param("group_id")?.parse()?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 4{
        let class = school_auth.school.get_class(&req, group_id, class_id).await?;
        class.del_act(&req).await?;
        let mut res = tide::Response::new(StatusCode::Ok);
        res.set_body(Body::from_json(&act_id)?);
        Ok(res)
    }
    else{
        let res = tide::Response::new(StatusCode::NotAcceptable);
        Ok(res)
    }
}

pub async fn limitations(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    //let school_id: i32 = req.param("school")?.parse()?;
    let class_id: i32 = req.param("class_id")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    let _class: Class = sqlx::query_as("SELECT * FROM classes WHERE id = $1 and school = $2")
        .bind(&class_id)
        .bind(&school_auth.school.id)
        .fetch_one(&req.state().db_pool).await?;
    if school_auth.role < 4 {
        match req.method() {
            Method::Get => {
                let mut class_availables = sqlx::query(r#"SELECT
                        days.id, days.name, class_available.hours
                        FROM class_available inner join days on class_available.day = days.id
                        WHERE class_available.class_id = $1"#)
                    .bind(&class_id)
                    .fetch(&req.state().db_pool);
                let mut availables: Vec<ClassAvailable> = Vec::new();
                while let Some(row) = class_availables.next().await? {
                    let available = ClassAvailable {
                        day: Day {
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
            }
            Method::Post => {
                let post = req.body_json::<Vec<ClassAvailable>>().await?;
                for available in post {
                    let update: sqlx::Result<InsertClassAvailable> = sqlx::query_as(r#"update class_available set hours = $3 where class_id= $1 and day = $2 returning class_id, hours, day"#)
                        .bind(&class_id)
                        .bind(&available.day.id)
                        .bind(&available.hours)
                        .fetch_one(&req.state().db_pool).await;
                    match update {
                        Ok(_s) => {}
                        Err(_) => {
                            let _insert: InsertClassAvailable = sqlx::query_as(r#"insert into class_available(class_id, day, hours) values($1, $2, $3) returning class_id, day, hours"#)
                                .bind(&class_id)
                                .bind(&available.day.id)
                                .bind(&available.hours)
                                .fetch_one(&req.state().db_pool).await?;
                        }
                    }
                }
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

pub async fn timetables(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let class_id = req.param("class_id")?;
    use sqlx_core::postgres::PgQueryAs;
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    let _class: Class = sqlx::query_as("SELECT * FROM classes WHERE id = $1 and school = $2")
        .bind(&class_id.parse::<i32>()?)
        .bind(&school_auth.school.id)
        .fetch_one(&req.state().db_pool).await?;
    if school_auth.role <= 8 {
        let mut class = sqlx::query("SELECT class_timetable.id, class_timetable.class_id, class_timetable.day_id, class_timetable.hour,
                            activities.id, users.id, users.first_name, users.last_name, subjects.name, users.is_active
                            FROM class_timetable inner join activities on class_timetable.activities = activities.id
                            inner join users on activities.teacher = users.id
                            inner join subjects on activities.subject = subjects.id WHERE class_id = $1")
            .bind(&class_id.parse::<i32>()?)
            .fetch(&req.state().db_pool);
        let mut class_timetables: Vec<ClassTimetable> = Vec::new();
        while let Some(row) = class.next().await? {
            let class_timetable = ClassTimetable {
                id: row.get(0),
                class_id: row.get(1),
                day_id: row.get(2),
                hour: row.get(3),
                activity: ClassTimetableActivity {
                    id: row.get(4),
                    teacher: Teacher {
                        id: row.get(5),
                        first_name: row.get(6),
                        last_name: row.get(7),
                        role_id: 0,
                        role_name: "".to_string(),
                        is_active: row.get(9)
                    }
                },
                subject: row.get(8)
            };
            class_timetables.push(class_timetable);
        }
        res.set_body(Body::from_json(&class_timetables)?);
        Ok(res)
    } else {
        Ok(res)
    }
}

pub async fn update_class(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_id: i32 = req.param("school")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    let class = req.body_json::<UpdateClass>().await?;
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    let mut s = SchoolDetail::default();
    let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, city.pk, city.name, town.pk, town.name \
            FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk WHERE school.id = $1")
        .bind(&school_id)
        .fetch(&req.state().db_pool);
    while let Some(row) = query.next().await?{
        s = SchoolDetail{
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
        Some(u) => {
            if s.manager == u.id || u.is_admin{
                let c: Class = sqlx::query_as("update classes set sube = $1, kademe = $2, school = $3, group_id = $4 where id = $5 returning id, sube, kademe, group_id, school")
                    .bind(&class.sube)
                    .bind(&class.kademe)
                    .bind(&school_id)
                    .bind(&class.group_id)
                    .bind(&class.id)
                    .fetch_one(&req.state().db_pool).await?;
                res.set_body(Body::from_json(&c)?);
                Ok(res)
            }
            else{
                Ok(res)
            }
        }
        None=>{
            Ok(res)
        }
    }
}

pub async fn get_students(req: Request<AppState>) -> tide::Result{
    let mut res = tide::Response::new(StatusCode::Ok);
    let class_id = req.param("class_id")?;
    let group_id = req.param("group_id")?;
    use sqlx_core::postgres::PgQueryAs;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 8 {
        let students: Vec<SimpleStudent> = sqlx::query_as(r#"SELECT students.id, students.first_name, students.last_name, students.school_number
         FROM students inner join class_student on students.id = class_student.student WHERE class_student.class_id = $1 and class_student.group_id = $2"#)
            .bind(&class_id.parse::<i32>()?)
            .bind(&group_id.parse::<i32>()?)
            .fetch_all(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&students)?);
    }
    Ok(res)
}

pub async fn get_all_students(req: Request<AppState>) -> tide::Result{
    let mut res = tide::Response::new(StatusCode::Ok);
    //let class_id = req.param("class_id")?;
    //let group_id = req.param("group_id")?;
    use sqlx_core::postgres::PgQueryAs;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 8 {
        let students: Vec<SimpleStudent> = sqlx::query_as(r#"SELECT first_name, last_name, id, school_number FROM students WHERE school = $1 and id not in (select student from class_student)"#)
            .bind(&school_auth.school.id)
            .fetch_all(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&students)?);
    }
    Ok(res)
}

pub async fn students(mut req: Request<AppState>) -> tide::Result{
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 6 {
        let student = req.body_json::<SimpleStudent>().await?;
        let class_id = req.param("class_id")?;
        let group_id = req.param("group_id")?;
        let _ = sqlx::query(r#"insert into class_student(student, class_id, group_id) values($1, $2, $3)"#)
            .bind(&student.id)
            .bind(&class_id.parse::<i32>()?)
            .bind(&group_id.parse::<i32>()?)
            .execute(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&student)?);
    }
    Ok(res)
}

pub async fn del_student(req: Request<AppState>) -> tide::Result{
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 6 {
        let class_id = req.param("class_id")?;
        let group_id = req.param("group_id")?;
        let student_id = req.param("student_id")?;
        let _ = sqlx::query(r#"delete from class_student where student = $1 and class_id = $2 and group_id = $3"#)
            .bind(&student_id.parse::<i32>()?)
            .bind(&class_id.parse::<i32>()?)
            .bind(&group_id.parse::<i32>()?)
            .execute(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&student_id.parse::<i32>()?)?);
    }
    Ok(res)
}