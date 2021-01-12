use tide::Request;
use crate::AppState;
use crate::request::{Auth, SchoolAuth};
use http_types::{StatusCode, Method, Body};
use crate::model::school;
use crate::model::user;
use crate::model::class;
use serde::*;
use crate::model::class::{NewClass};
use crate::model::timetable::{NewTimetable};
use crate::model::school::{SchoolDetail, School};
use crate::model::post::SchoolPost;
use crate::model::city::{City, Town};
use async_std::{fs::OpenOptions, io};
use crate::model::student::{NewStudent, Student};
use crate::model::subject;
use crate::model::class_room;


pub async fn schools(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    //let mut school: Vec<school::SchoolDetail> = Vec::new();
    match req.user().await {
        Some(u)=> {
            use sqlx_core::cursor::Cursor;
            use sqlx_core::row::Row;
            let mut s: Vec<SchoolDetail> = vec![];
            let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, city.pk, city.name, town.pk, town.name \
                    FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk \
                    inner join school_users on school_users.school_id = school.id WHERE school_users.user_id = $1")
                .bind(&u.id)
                .fetch(&req.state().db_pool);
            while let Some(row) = query.next().await?{
                let school = SchoolDetail{
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
                };
                s.push(school)
            }
            res.set_body(Body::from_json(&s)?);
            res.insert_header("content-type", "application/json");
            Ok(res)
        }
        None=>{
            Ok(res)
        }
    }
}
pub async fn school_type(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    use crate::model::school::{SchoolType};
    match req.user().await{
        Some(_)=>{
            let s: Vec<SchoolType> = sqlx::query_as("SELECT * FROM school_type")
                .fetch_all(&req.state().db_pool).await?;
            res.set_body(Body::from_json(&s)?);
            Ok(res)
        }
        None=>{
            Ok(res)
        }
    }

}
pub async fn add(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    match req.user().await{
        Some(u)=>{
            let s: Result<school::School, sqlx_core::error::Error> = sqlx::query_as("SELECT * FROM school WHERE manager = $1")
                .bind(&u.id)
                .fetch_one(&req.state().db_pool).await;
            match s {
                Ok(_s) => {
                    res.set_body(Body::from_json(&"Kayıtlı kurumunuz mevcut")?);
                    Ok(res)
                }
                Err(_) => {
                    let form = req.body_json::<school::NewSchool>().await?;
                    let add_school: sqlx::Result<School> = sqlx::query_as(r#"INSERT into school (name, town, school_type, manager) values($1, $2, $3, $4) returning id, name, manager"#)
                        .bind(&form.name).bind(&form.town).bind(&form.school_type).bind(&u.id)
                        .fetch_one(&req.state().db_pool).await;
                    match add_school {
                        Ok(_school) => {
                            use sqlx_core::cursor::Cursor;
                            use sqlx_core::row::Row;
                            let mut s = SchoolDetail::default();
                            let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, city.pk, city.name, town.pk, town.name \
                                    FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk WHERE school.id = $1")
                                .bind(&_school.id)
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
                            let _add_school_user = sqlx::query!(r#"INSERT into school_users (school_id, user_id, role) values($1, $2, 1)"#,
                                    s.id, u.id)
                                .execute(&req.state().db_pool).await?;
                            let mut hour: i32 = 7;
                            if s.school_type == 3{
                                hour = 8;
                            }
                            let _g = sqlx::query!(r#"insert into class_groups (name, school, hour) values($1, $2, $3)"#, &"Varsayılan", s.id, hour)
                                .execute(&req.state().db_pool).await;
                            res.set_body(Body::from_json(&s)?);
                            Ok(res)
                        }
                        Err(_e) => {
                            res.set_body(Body::from_json(&"Bu eposta ile kaydedilmiş okul mevcut")?);
                            //res = res.set_status(StatusCode::NotAcceptable);
                            Ok(res)
                        }
                    }
                }
            }
        }
        None=>{
            Ok(res)
        }
    }

}

pub async fn school_detail(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    res.set_body(Body::from_json(&(&school_auth.role, &school_auth.school))?);
    Ok(res)
}

pub async fn patch_school(mut req: Request<AppState>) -> tide::Result {
    let form: school::UpdateSchoolForm = req.body_json::<school::UpdateSchoolForm>().await?;
    let school: &SchoolDetail = req.ext().unwrap();
    let res = tide::Response::new(StatusCode::Ok);
    let _ = sqlx::query("update school set name = $1, tel = $3, location = $4 where id = $2")
        .bind(&form.name)
        .bind(&school.id)
        .bind(&form.tel)
        .bind(&form.location)
        .execute(&req.state().db_pool).await?;
    Ok(res)
}
pub async fn classes(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    //let school_id: i32 = req.param("school")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    let classes: Vec<class::Class> = sqlx::query_as("SELECT * FROM classes WHERE school = $1 order by kademe, sube")
        .bind(&school_auth.school.id)
        .fetch_all(&req.state().db_pool).await?;
    //println!("{:?}", &classes);
    res.set_body(Body::from_json(&classes)?);
    Ok(res)
}
pub async fn add_class(mut req: Request<AppState>) -> tide::Result {
    let class = req.body_json::<NewClass>().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 6 {
        use sqlx_core::postgres::PgQueryAs;
        let mut res = tide::Response::new(StatusCode::Ok);
        let c: class::Class = sqlx::query_as("insert into classes(sube, kademe, school, group_id) values($1, $2, $3, $4) returning id, sube, kademe, school, group_id ")
            .bind(&class.sube)
            .bind(&class.kademe)
            .bind(&school_auth.school.id)
            .bind(&class.group_id)
            .fetch_one(&req.state().db_pool).await?;
        let days: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        let group: ClassGroups = sqlx::query_as("SELECT * FROM class_groups WHERE id = $1")
            .bind(&c.group_id)
            .fetch_one(&req.state().db_pool).await?;
        for d in days {
            let hours: Vec<bool>;
            if d > 5 {
                hours = vec![false; group.hour as usize];
            } else {
                hours = vec![true; group.hour as usize];
            }
            let _class_available = sqlx::query("INSERT into class_available(class_id,  day, hours) values($1, $2, $3)")
                .bind(&c.id)
                .bind(d)
                .bind(hours)
                .execute(&req.state().db_pool).await?;
        }
        res.set_body(Body::from_json(&c)?);
        Ok(res)
    } else {
        let res = tide::Response::new(StatusCode::Unauthorized);
        Ok(res)
    }
}
pub async fn get_subjects(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    use sqlx::prelude::PgQueryAs;
    let subjects: Vec<subject::Subject> = sqlx::query_as("SELECT * FROM subjects WHERE school = $1 order by optional, name, kademe")
        .bind(&school_auth.school.id)
        .fetch_all(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&subjects)?);
    Ok(res)
}
pub async fn subjects(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let _subject = req.body_json::<subject::NewSubject>().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    use sqlx::prelude::PgQueryAs;
    if school_auth.role < 3 && school_auth.school.id == _subject.school{
        let s: subject::Subject  = sqlx::query_as("insert into subjects(name, school, optional, kademe) values($1, $2, $3, $4) returning id, name, school, optional, kademe")
            .bind(&_subject.name)
            .bind(&school_auth.school.id)
            .bind(&_subject.optional)
            .bind(&_subject.kademe)
            .fetch_one(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&s)?);
        Ok(res)
    }
    else {
        Ok(res)
    }
}

pub async fn del_subject(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    use sqlx::prelude::PgQueryAs;
    let subject_id: i32 = req.param("subject_id")?.parse()?;
    if school_auth.role < 3 {
        let _ = sqlx::query("delete from subjects where school = $1 and id = $2")
            .bind(&school_auth.school.id)
            .bind(&subject_id)
            .execute(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&subject_id)?);
        Ok(res)
    }
    else {
        Ok(res)
    }
}

pub async fn teachers(mut req: Request<AppState>) -> tide::Result {
    //use crate::request::SchoolAuthExt;
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    match req.method() {
        Method::Get => {
            let school_auth: &SchoolAuth = req.ext().unwrap();
            if let Some(user) = req.user().await{
                use sqlx::Cursor;
                use sqlx::Row;
                let mut tchrs = sqlx::query("SELECT users.id, users.first_name, users.last_name, roles.id, roles.name \
                        FROM school_users inner join users on school_users.user_id = users.id inner join roles on school_users.role = roles.id \
                        WHERE school_users.school_id = $1 and school_users.role <= 5 order by roles.id, users.first_name")
                    .bind(&school_auth.school.id)
                    .fetch(&req.state().db_pool);
                let mut teachers: Vec<user::Teacher> = vec![];
                while let Some(row) = tchrs.next().await?{
                    let teacher = user::Teacher{
                        id: row.get(0),
                        first_name: row.get(1),
                        last_name: row.get(2),
                        role_id: row.get(3),
                        role_name: row.get(4)
                    };
                    teachers.push(teacher);
                }
                res.set_body(Body::from_json(&teachers)?);
                Ok(res)
            }
            else {
                let res = tide::Response::new(StatusCode::Unauthorized);
                Ok(res)
            }
        }
        Method::Post => {
            #[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
            struct NewTeacher {
                first_name: String,
                last_name: String,
                role: i16
            }
            let form = req.body_json::<NewTeacher>().await;
            let school_auth: &SchoolAuth = req.ext().unwrap();

            match form {
                Ok(f) => {
                    if school_auth.role < 3 && f.role != 1{
                        let add_user: user::SimpleUser = sqlx::query_as("INSERT into users(first_name, last_name) values($1, $2) returning id, first_name, last_name, email, is_admin, username")
                            .bind(&f.first_name)
                            .bind(&f.last_name)
                            .fetch_one(&req.state().db_pool).await?;
                        let _add_teacher = sqlx::query("INSERT into school_users(user_id, school_id, role) values($1, $2, $3)")
                            .bind(&add_user.id)
                            .bind(&school_auth.school.id)
                            .bind(&f.role)
                            .execute(&req.state().db_pool).await?;
                        let days: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];

                        for d in days {
                            let groups: Vec<ClassGroups> = sqlx::query_as("SELECT * FROM class_groups WHERE school = $1")
                                .bind(&school_auth.school.id)
                                .fetch_all(&req.state().db_pool).await?;
                            for g in groups {
                                let hours: Vec<bool>;
                                if d > 5 {
                                    hours = vec![false; g.hour as usize];
                                } else {
                                    hours = vec![true; g.hour as usize];
                                }
                                let _teacher_available = sqlx::query("INSERT into teacher_available(user_id, school_id, day, hours, group_id) values($1, $2, $3, $4, $5)")
                                    .bind(&add_user.id)
                                    .bind(&school_auth.school.id)
                                    .bind(d)
                                    .bind(hours)
                                    .bind(&g.id)
                                    .execute(&req.state().db_pool).await;
                            }
                        }
                        use sqlx::Cursor;
                        use sqlx::Row;
                        let mut tchrs = sqlx::query("SELECT users.id, users.first_name, users.last_name, roles.id, roles.name \
                        FROM school_users inner join users on school_users.user_id = users.id inner join roles on school_users.role = roles.id \
                        WHERE school_users.user_id = $1")
                            .bind(&add_user.id)
                            .fetch(&req.state().db_pool);
                        let mut teacher: user::Teacher = user::Teacher::default();
                        while let Some(row) = tchrs.next().await?{
                            teacher = user::Teacher{
                                id: row.get(0),
                                first_name: row.get(1),
                                last_name: row.get(2),
                                role_id: row.get(3),
                                role_name: row.get(4)
                            };
                            res.set_body(Body::from_json(&teacher)?);
                        }
                        Ok(res)
                    }
                    else {
                        let res = tide::Response::new(StatusCode::Unauthorized);
                        Ok(res)
                    }
                }
                Err(_) => {
                    Ok(res)
                }
            }
        }
        _ => {
            Ok(res)
        }
    }
}

pub async fn students(mut req: Request<AppState>) -> tide::Result {
    let student = req.body_json::<NewStudent>().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 5 {
        use sqlx::prelude::PgQueryAs;
        let mut res = tide::Response::new(StatusCode::Ok);
        let s: user::Student = sqlx::query_as("insert into users(first_name, last_name) values($1, $2) returning id, first_name, last_name")
            .bind(&student.first_name)
            .bind(&student.last_name)
            .fetch_one(&req.state().db_pool).await?;
        let _s = sqlx::query("insert into school_users(school_id, user_id, role) values($1, $2, 7)")
            .bind(&school_auth.school.id)
            .bind(&s.id)
            .execute(&req.state().db_pool).await?;
        let s: Student = sqlx::query_as("insert into students(first_name, last_name, school, school_number, user_id) values($1, $2, $3, $4, $5) returning id, first_name, last_name, school, school_number ")
            .bind(&student.first_name)
            .bind(&student.last_name)
            .bind(&school_auth.school.id)
            .bind(&student.number)
            .bind(&s.id)
            .fetch_one(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&s)?);
        Ok(res)
    }
    else {
        let res = tide::Response::new(StatusCode::Unauthorized);
        Ok(res)
    }
}

pub async fn class_rooms(mut req: Request<AppState>) -> tide::Result {
    let class_room = req.body_json::<class_room::NewClassroom>().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 3 {
        use sqlx::prelude::PgQueryAs;
        let mut res = tide::Response::new(StatusCode::Ok);
        let cls: class_room::Classroom = sqlx::query_as("insert into class_rooms(name, school, rw, cl, width) values($1, $2, $3, $4, $5) returning id, name, school, rw, cl, width")
            .bind(&class_room.name)
            .bind(&class_room.school)
            .bind(&class_room.rw)
            .bind(&class_room.cl)
            .bind(&class_room.width)
            .fetch_one(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&cls)?);
        Ok(res)
    }
    else {
        let res = tide::Response::new(StatusCode::Unauthorized);
        Ok(res)
    }
}

pub async fn get_class_rooms(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    //let student = req.body_json::<NewStudent>().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    use sqlx::prelude::PgQueryAs;
    if school_auth.role < 8{
        let students: Vec<class_room::Classroom> = sqlx::query_as("select * from class_rooms where school = $1")
            .bind(&school_auth.school.id)
            .fetch_all(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&students)?);
        Ok(res)
    }
    else {
        Ok(res)
    }
}

pub async fn del_class_room(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let number = req.param("class_room_id").unwrap().parse::<i32>().unwrap();
    let school_auth: &SchoolAuth = req.ext().unwrap();
    //use sqlx::prelude::PgQueryAs;
    if school_auth.role < 3{
        let _ = sqlx::query("delete from class_rooms where id = $1 and school = $2")
            .bind(&number)
            .bind(&school_auth.school.id)
            .execute(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&number)?);
    }

    Ok(res)
}

pub async fn get_unused_numbers(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    use sqlx::prelude::PgQueryAs;
    let numbers: (Option<Vec<i32>>,) = sqlx::query_as("select array_agg(school_number) from students where school = $1")
        .bind(&school_auth.school.id)
        .fetch_one(&req.state().db_pool).await?;
    let mut unused_numbers: Vec<i32> = vec![];
    let mut s = 1;
    match numbers.0{
        Some(n) => {
            loop{
                if n.iter().all(|nn| nn != &s){
                    unused_numbers.push(s.clone());
                }
                if unused_numbers.len() >= 10{
                    break;
                }
                s += 1;
            }
        }
        None => {}
    }

    res.set_body(Body::from_json(&unused_numbers)?);
    Ok(res)
}
pub async fn get_students(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    //let student = req.body_json::<NewStudent>().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    use sqlx::prelude::PgQueryAs;
    if school_auth.role < 8{
        let students: Vec<Student> = sqlx::query_as("select * from students where school = $1")
            .bind(&school_auth.school.id)
            .fetch_all(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&students)?);
        Ok(res)
    }
    else {
        Ok(res)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Students{
    file: Vec<u8>,
    group: i32
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct ClassGroups{
    id: i32,
    name: String,
    hour: i32
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct AddGroup{
    name: String,
    hour: i32
}


pub async fn get_posts(req: Request<AppState>) -> tide::Result {
    use crate::model::post::Post;
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    let school_id: i32 = req.param("school")?.parse()?;
    let posts: Vec<Post> = sqlx::query_as("SELECT * from post where school = $1 order by pub_date desc limit 40")
        .bind(&school_id)
        .fetch_all(&req.state().db_pool).await?;
    use sqlx_core::cursor::Cursor;
    use sqlx_core::row::Row;
    let mut sch = SchoolDetail::default();
    let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, city.pk, city.name, town.pk, town.name \
            FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk WHERE school.id = $1")
        .bind(&school_id)
        .fetch(&req.state().db_pool);
    while let Some(row) = query.next().await?{
        sch = SchoolDetail{
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
    let mut school_posts: Vec<SchoolPost> = vec![];
    for p in posts {
        let school_post = SchoolPost {
            id: p.id,
            body: p.body,
            pub_date: p.pub_date,
            school: Some(sch.clone()),
            sender: p.sender
        };
        school_posts.push(school_post);
    }
    res.set_body(Body::from_json(&school_posts)?);
    Ok(res)
}

pub async fn city(req: Request<AppState>) -> tide::Result {
    let school_id: i32 = req.param("school")?.parse()?;
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    let cities: Vec<City> = sqlx::query_as("SELECT city.pk, city.name FROM city inner join school where school.id = $1 ")
        .bind(&school_id)
        .fetch_all(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&cities)?);
    Ok(res)
}

