use serde::*;
use tide::{Request, Response};
use http_types::{StatusCode, Body};
use crate::AppState;
use crate::request::Auth;
use crate::model::school::{SchoolDetail, School};
use crate::model::city::{City, Town};
use chrono::NaiveTime;
use crate::model::timetable;
use crate::model::timetable::NewTimetable;


pub async fn groups(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_id: i32 = req.param("school")?.parse()?;
    //let mut school: Vec<school::SchoolDetail> = Vec::new();
    use sqlx_core::postgres::PgQueryAs;
    let s: Vec<ClassGroups> = sqlx::query_as("SELECT * FROM class_groups WHERE school = $1")
        .bind(&school_id)
        .fetch_all(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&s)?);
    res.insert_header("content-type", "application/json");
    Ok(res)
}

pub async fn get_group(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_id: i32 = req.param("school")?.parse()?;
    let group_id: i32 = req.param("group_id")?.parse()?;
    //let mut school: Vec<school::SchoolDetail> = Vec::new();

    use sqlx_core::postgres::PgQueryAs;
    let s: ClassGroups = sqlx::query_as("SELECT * FROM class_groups WHERE school = $1 and id = $2")
        .bind(&school_id)
        .bind(&group_id)
        .fetch_one(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&s)?);
    res.insert_header("content-type", "application/json");
    Ok(res)
}

pub async fn add_groups(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let group = req.body_json::<AddGroup>().await?;
    let school_id: i32 = req.param("school")?.parse()?;
    //let mut school: Vec<school::SchoolDetail> = Vec::new();
    match req.user().await {
        Some(u)=> {
            use sqlx_core::postgres::PgQueryAs;
            use sqlx_core::cursor::Cursor;
            use sqlx_core::row::Row;
            let mut _school = SchoolDetail::default();
            let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, city.pk, city.name, town.pk, town.name \
                    FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk WHERE school.id = $1")
                .bind(&school_id)
                .fetch(&req.state().db_pool);
            while let Some(row) = query.next().await?{
                _school = SchoolDetail{
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
            if u.is_admin || _school.manager == u.id{
                let s: ClassGroups = sqlx::query_as("insert into class_groups(school, name, hour) values($1, $2, $3) returning id, name, hour")
                    .bind(&school_id)
                    .bind(&group.name)
                    .bind(&group.hour)
                    .fetch_one(&req.state().db_pool).await?;
                use crate::model::school::SchoolTeacher;
                let teachers: Vec<SchoolTeacher> = sqlx::query_as("select * from school_users where school_id = $1")
                    .bind(&school_id)
                    .fetch_all(&req.state().db_pool).await?;
                let days: Vec<i32> = vec![1,2,3,4,5, 6,7];
                for t in teachers{
                    for d in &days{
                        let hours: Vec<bool>;
                        if d > &5{
                            hours = vec![false; s.hour as usize];
                        }
                        else{
                            hours = vec![true; s.hour as usize];
                        }
                        let _teacher_available = sqlx::query("INSERT into teacher_available(user_id, school_id, day, hours, group_id) values($1, $2, $3, $4, $5)")
                            .bind(&t.user_id)
                            .bind(&school_id)
                            .bind(d)
                            .bind(hours)
                            .bind(&s.id)
                            .execute(&req.state().db_pool).await;

                    }
                }
                res.set_body(Body::from_json(&s)?);
                res.insert_header("content-type", "application/json");
                Ok(res)
            }
            else {
                Ok(res)
            }
        }
        None=>{
            Ok(res)
        }
    }
}

pub async fn del_group(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_id: i32 = req.param("school")?.parse()?;
    let group_id: i32 = req.param("group_id")?.parse()?;
    match req.user().await {
        Some(u)=> {
            use sqlx_core::postgres::PgQueryAs;
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
            if u.is_admin || school.manager == u.id{
                let groups: Vec<ClassGroups> = sqlx::query_as("select * from class_groups where school = $1")
                    .bind(&school_id)
                    .fetch_all(&req.state().db_pool).await?;

                if groups.len() > 1 {
                    let _del_teacher_availables = sqlx::query("delete from teacher_available where school_id = $1 and group_id = $2")
                        .bind(&school_id)
                        .bind(&group_id)
                        .execute(&req.state().db_pool).await?;
                    let s: ClassGroups = sqlx::query_as("delete from class_groups where school = $1 and id = $2 returning id, name, hour")
                        .bind(&school_id)
                        .bind(&group_id)
                        .fetch_one(&req.state().db_pool).await?;
                    res.set_body(Body::from_json(&s)?);
                    res.insert_header("content-type", "application/json");
                    Ok(res)
                } else {
                    res.insert_header("content-type", "application/json");
                    Ok(res)
                }
            }
            else {
                res.insert_header("content-type", "application/json");
                Ok(res)
            }
        }
        None=>{
            Ok(res)
        }
    }
}

pub async fn patch_group(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let group_form = req.body_json::<AddGroup>().await?;
    let school_id: i32 = req.param("school")?.parse()?;
    let group_id: i32 = req.param("group_id")?.parse()?;
    match req.user().await {
        Some(u)=> {
            use sqlx_core::postgres::PgQueryAs;
            use sqlx_core::cursor::Cursor;
            use sqlx_core::row::Row;
            let mut _school = SchoolDetail::default();
            let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, city.pk, city.name, town.pk, town.name \
                    FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk WHERE school.id = $1")
                .bind(&school_id)
                .fetch(&req.state().db_pool);
            while let Some(row) = query.next().await?{
                _school = SchoolDetail{
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
            if u.is_admin || _school.manager == u.id{
                let s: ClassGroups = sqlx::query_as("update class_groups set hour = $3, name = $4 where school = $1 and id = $2 returning id, name, hour")
                    .bind(&school_id)
                    .bind(&group_id)
                    .bind(&group_form.hour)
                    .bind(&group_form.name)
                    .fetch_one(&req.state().db_pool).await?;
                println!("{:?}", &s);
                res.set_body(Body::from_json(&s)?);
                Ok(res)
            }
            else {
                Ok(res)
            }
        }
        None=>{
            Ok(res)
        }
    }
}

pub async fn group_schedules(req: Request<AppState>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    let school_id: i32 = req.param("school")?.parse()?;
    let group_id: i32 = req.param("group_id")?.parse()?;
    match req.user().await {
        Some(_u)=> {
            use sqlx_core::postgres::PgQueryAs;
            let group: ClassGroups = sqlx::query_as("SELECT * from class_groups WHERE id = $1 and school = $2")
                .bind(&group_id)
                .bind(&school_id)
                .fetch_one(&req.state().db_pool).await?;
            let mut schedules:  Vec<Schedules>= sqlx::query_as("SELECT * from group_schedules WHERE group_id = $1 order by hour")
                .bind(&group_id)
                .fetch_all(&req.state().db_pool).await?;
            if schedules.len() != group.hour as usize{
                //schedules.clear();
                for s in 0..group.hour{
                    match schedules.iter().find(|ss| ss.hour == s+1){
                        Some(_) => {}
                        None => {
                            let schdls = Schedules{
                                group_id: group_id,
                                hour: (s+1) as i32,
                                start_time: NaiveTime::parse_from_str("00:00", "%H:%M").unwrap(),
                                end_time: NaiveTime::parse_from_str("00:00", "%H:%M").unwrap()
                            };
                            schedules.push(schdls)
                        }
                    }
                }
            }
            res.set_body(Body::from_json(&schedules)?);
            Ok(res)
        }
        None => {
            Ok(res)
        }
    }
}

pub async fn patch_group_schedules(mut req: Request<AppState>) -> tide::Result {
    let res = Response::new(StatusCode::Ok);
    match req.user().await {
        Some(u)=> {
            let school_id: i32 = req.param("school")?.parse()?;
            let group_id: i32 = req.param("group_id")?.parse()?;
            use sqlx_core::postgres::PgQueryAs;
            let _school: School = sqlx::query_as("SELECT * from school WHERE id = $1 and manager = $2")
                .bind(&school_id)
                .bind(u.id)
                .fetch_one(&req.state().db_pool).await?;
            let _group: ClassGroups = sqlx::query_as("SELECT * from class_groups WHERE id = $1 and school = $2")
                .bind(&group_id)
                .bind(&school_id)
                .fetch_one(&req.state().db_pool).await?;
            let schedules_form = req.body_json::<Vec<Schedules>>().await?;
            for s in schedules_form{
                let _schedules = sqlx::query("update group_schedules set start_time = $3, end_time = $4 WHERE group_id = $1 and hour = $2")
                    .bind(&s.group_id)
                    .bind(&s.hour)
                    .bind(&s.start_time)
                    .bind(&s.end_time)
                    .execute(&req.state().db_pool).await?;
                if _schedules == 0 {
                    let _schedules2 = sqlx::query("insert into group_schedules(group_id, hour, start_time, end_time) values($1, $2, $3, $4)")
                        .bind(&s.group_id)
                        .bind(&s.hour)
                        .bind(&s.start_time)
                        .bind(&s.end_time)
                        .execute(&req.state().db_pool).await?;
                }
            }
            Ok(res)
        }
        None => {
            Ok(res)
        }
    }
}

pub async fn timetables(mut req: Request<AppState>) -> tide::Result {
    let res = tide::Response::new(StatusCode::Ok);
    let school_id: i32 = req.param("school")?.parse()?;
    let group_id: i32 = req.param("group_id")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    let mut school = SchoolDetail::default();
    let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, school.tel, school.location, city.pk, city.name, town.pk, town.name \
            FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk WHERE school.id = $1")
        .bind(&school_id)
        .fetch(&req.state().db_pool);
    while let Some(row) = query.next().await?{
        school = SchoolDetail{
            id: row.get(0),
            name: row.get(1),
            manager: row.get(2),
            school_type: row.get(3),
            tel: row.get(4),
            location: row.get(5),
            city: City{ pk: row.get(6), name: row.get(7) },
            town: Town{
                city: row.get(6),
                pk: row.get(8),
                name: row.get(9)
            }
        }
    }
    match req.user().await {
        Some(user) => {
            if user.id == school.manager || user.is_admin{
                let posts = req.body_json::<Vec<timetable::NewTimetable>>();
                match posts.await{
                    Ok(post)=> {
                        //for p in &post{
                        let classes: (Option<Vec<i32>>, ) = sqlx::query_as(r#"select array_agg(id) from  classes where school= $1 and group_id = $2"#)
                            .bind(&school_id)
                            .bind(&group_id)
                            .fetch_one(&req.state().db_pool).await?;
                        sqlx::query("delete from class_timetable using activities where array[$1::int[]] @> activities.classes ")
                            .bind(&classes.0.unwrap())
                            .execute(&req.state().db_pool).await?;
                        for p in post {
                            let _insert: timetable::NewTimetable = sqlx::query_as("insert into class_timetable(class_id, day_id, hour, activities) values($1, $2, $3, $4) returning class_id, day_id, hour, activities")
                                .bind(&p.class_id)
                                .bind(&p.day_id)
                                .bind(&p.hour)
                                .bind(&p.activities)
                                .fetch_one(&req.state().db_pool).await?;
                        }
                        Ok(res)
                    }
                    Err(_)=>{
                        Ok(res)
                    }
                }
            }
            else{
                Ok(res)
            }
        }
        None => {
            Ok(res)
        }
    }
}

pub async fn get_timetables(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let group_id: i32 = req.param("group_id")?.parse()?;
    let school_id: i32 = req.param("school")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    let mut school = SchoolDetail::default();
    let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, school.tel, school.location, city.pk, city.name, town.pk, town.name \
            FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk WHERE school.id = $1")
        .bind(&school_id)
        .fetch(&req.state().db_pool);
    while let Some(row) = query.next().await?{
        school = SchoolDetail{
            id: row.get(0),
            name: row.get(1),
            manager: row.get(2),
            school_type: row.get(3),
            tel: row.get(4),
            location: row.get(5),
            city: City{ pk: row.get(6), name: row.get(7) },
            town: Town{
                city: row.get(6),
                pk: row.get(8),
                name: row.get(9)
            }
        }
    }
    match req.user().await {
        Some(user) => {
            use crate::model::timetable::{TeacherAvailable, TimetableData, ClassAvailable2, Class, Activity, Teacher};
            if user.id == school.manager || user.is_admin{
                let tat: Vec<TeacherAvailable> = sqlx::query_as(r#"select * from teacher_available where school_id = $1 and group_id = $2"#)
                    .bind(&school_id)
                    .bind(&group_id)
                    .fetch_all(&req.state().db_pool).await?;
                let teachers: Vec<Teacher> = sqlx::query_as(r#"select * from users inner join school_users on users.id = school_users.user_id where school_users.school_id = $1"#)
                    .bind(&school_id)
                    .fetch_all(&req.state().db_pool).await?;
                let cat: Vec<ClassAvailable2> = sqlx::query_as(r#"select * from class_available inner join classes on class_available.class_id = classes.id
                        where classes.school = $1 and classes.group_id = $2"#)
                    .bind(&school_id)
                    .bind(&group_id)
                    .fetch_all(&req.state().db_pool).await?;
                let classes: Vec<Class> = sqlx::query_as(r#"select * from classes where school = $1 and group_id = $2"#)
                    .bind(&school_id)
                    .bind(&group_id)
                    .fetch_all(&req.state().db_pool).await?;
                let classes_id: (Vec<i32>,) = sqlx::query_as(r#"select array_agg(id) from classes where school = $1 and group_id = $2"#)
                    .bind(&school_id)
                    .bind(&group_id)
                    .fetch_one(&req.state().db_pool).await?;

                let acts: Vec<Activity> = sqlx::query_as(r#"select activities.id, activities.subject, activities.hour, activities.teacher, activities.split, activities.classes
                        from activities where classes && $1::integer[]"#)
                    //.bind(&school_id)
                    .bind(&classes_id.0)
                    .fetch_all(&req.state().db_pool).await?;
                let timetables: Vec<NewTimetable> = sqlx::query_as("SELECT class_timetable.class_id, class_timetable.day_id, class_timetable.hour, class_timetable.activities
                            FROM class_timetable inner join classes on class_timetable.class_id = classes.id
                            WHERE class_timetable.class_id = any($1) and classes.group_id = $2")
                    .bind(&classes_id.0)
                    .bind(&group_id)
                    .fetch_all(&req.state().db_pool).await?;
                let timetable_data = TimetableData{
                    tat,
                    cat,
                    acts,
                    classes,
                    teachers,
                    timetables
                };
                res.set_body(Body::from_json(&timetable_data)?);
                Ok(res)
            } else {
                Ok(res)
            }
        }
        None => {
            Ok(res)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct ClassGroups{
    id: i32,
    name: String,
    hour: i32
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