use tide::Request;
use crate::AppState;
use crate::request::{Auth, SchoolAuth};
use http_types::{StatusCode, Body};
use crate::model::school;
use crate::model::user;
use crate::model::group;
use serde::*;
use crate::model::school::{SchoolDetail};
use crate::model::post::SchoolPost;
use crate::model::city::{City, Town};
use crate::model::student::{NewStudent, Student};
use crate::model::{subject, library, class_room};
use multer::Multipart;
use crate::model::user::SimpleUser;
use crate::model::teacher::Teacher;


pub async fn schools(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let schools = req.get_schools().await?;
    res.set_body(Body::from_json(&schools)?);
    //res.insert_header("content-type", "application/json");
    Ok(res)
}
pub async fn school_type(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    use crate::model::school::{SchoolType};
    let _user = req.user().await?;
    let s: Vec<SchoolType> = sqlx::query_as("SELECT * FROM school_type")
        .fetch_all(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&s)?);
    Ok(res)
}
pub async fn add(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    let u = req.user().await?;
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
            let add_school = form.add(&mut req, u.id).await;
            match add_school {
                Ok(_school) => {
                    use sqlx_core::cursor::Cursor;
                    use sqlx_core::row::Row;
                    let mut s = SchoolDetail::default();
                    let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, city.pk, city.name, town.pk, town.name \
                                    FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk WHERE school.id = $1")
                        .bind(&_school.id)
                        .fetch(&req.state().db_pool);
                    while let Some(row) = query.next().await? {
                        s = SchoolDetail {
                            id: row.get(0),
                            name: row.get(1),
                            manager: row.get(2),
                            school_type: row.get(3),
                            tel: None,
                            location: None,
                            city: City { pk: row.get(4), name: row.get(5) },
                            town: Town {
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
                    if s.school_type == 3 {
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

pub async fn school_detail(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    res.set_body(Body::from_json(&(&school_auth.role, &school_auth.school))?);
    Ok(res)
}

pub async fn get_groups(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    let groups: Vec<group::ClassGroups> = school_auth.school.get_groups(&req).await?;
    res.set_body(Body::from_json(&groups)?);
    res.insert_header("content-type", "application/json");
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

pub async fn get_subjects(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    let subjects = school_auth.school.get_subjects(&req).await?;
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
pub async fn del_subject(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    let subject_id: i32 = req.param("subject_id")?.parse()?;
    if school_auth.role < 3 {
        let _ = sqlx::query("delete from subjects where school = $1 and id = $2")
            .bind(&school_auth.school.id)
            .bind(&subject_id)
            .execute(&req.state().db_pool).await?;
        let _ = sqlx::query("delete from activities where id = $1")
            .bind(&subject_id)
            .execute(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&subject_id)?);
        Ok(res)
    }
    else {
        Ok(res)
    }
}
pub async fn get_teachers(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 6 {
        let teachers = school_auth.school.get_teachers(&req).await?;
        res.set_body(Body::from_json(&teachers)?);
        Ok(res)
    } else {
        let res = tide::Response::new(StatusCode::Unauthorized);
        Ok(res)
    }
}
pub async fn teachers(mut req: Request<AppState>) -> tide::Result {
    //use crate::request::SchoolAuthExt;
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;

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
            if school_auth.role < 3 {
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
                let mut teacher: Teacher;
                while let Some(row) = tchrs.next().await? {
                    teacher = Teacher {
                        id: row.get(0),
                        first_name: row.get(1),
                        last_name: row.get(2),
                        role_id: row.get(3),
                        role_name: row.get(4),
                        is_active: false,
                        email: None,
                        tel: None
                    };
                    res.set_body(Body::from_json(&teacher)?);
                }
                Ok(res)
            } else {
                let res = tide::Response::new(StatusCode::Unauthorized);
                Ok(res)
            }
        }
        Err(_) => {
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
    } else {
        let res = tide::Response::new(StatusCode::Unauthorized);
        Ok(res)
    }
}
pub async fn students_with_file(req: Request<AppState>) -> tide::Result {
    // MultiPartFor------------
    use futures_codec::{BytesCodec, FramedRead};
    let content_type_string = req.header("Content-Type").unwrap().get(0).unwrap().as_str();
    let boundary_key = "boundary=";
    let skip_last_index = content_type_string.find(boundary_key).unwrap() + boundary_key.len();
    let boundary: String = content_type_string.chars().skip(skip_last_index).take(content_type_string.len() - skip_last_index).collect();
    let number = req.get_school().await.unwrap();
    let pool = &req.state().db_pool.clone();
    let stream = FramedRead::new(req, BytesCodec);
    let mut multipart = Multipart::new(stream, boundary);
    while let Some(mut field) = multipart.next_field().await? {
        // Get the field's filename if provided in "Content-Disposition" header.
        let file_name = field.file_name();
        if let Some(_file) = file_name {
            use async_std::fs::File;
            use async_std::prelude::*;
            //let school_auth: &SchoolAuth = req.ext().unwrap();

            while let Some(chunk) = field.chunk().await? {
                // Do something with field chunk.
                let mut file = File::create("students/".to_owned() + &number.name + ".xlsx").await?;
                file.write_all(&chunk).await?;
            }
        }
    }
    use calamine::{open_workbook, Xlsx, Reader};
    let mut excel: Xlsx<_> = open_workbook("students/".to_owned() + &number.name+".xlsx").unwrap();

    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            let title = row[1].get_float();
            if let Some(t) = title {
                let student = NewStudent {
                    first_name: row[4].get_string().unwrap().to_string(),
                    last_name: row[9].get_string().unwrap().to_string(),
                    number: t as i32
                };
                use sqlx::prelude::PgQueryAs;
                let user: SimpleUser = sqlx::query_as(r#"insert into users(first_name, last_name) values($1, $2) returning id, first_name, last_name"#)
                    .bind(&student.first_name)
                    .bind(&student.last_name)
                    .fetch_one(pool).await?;
                let _ = sqlx::query(r#"insert into school_users(user_id, school_id, role) values($1, $2, 8)"#)
                    .bind(&user.id)
                    .bind(&number.id)
                    .execute(pool).await?;
                let _ = sqlx::query(r#"insert into students(first_name, last_name, school, school_number, user_id) values($1, $2, $3, $4, $5)"#)
                    .bind(&student.first_name)
                    .bind(&student.last_name)
                    .bind(&number.id)
                    .bind(&student.number)
                    .bind(&user.id)
                    .execute(pool).await?;
            }
        }
    }
    let res = tide::Response::new(StatusCode::Ok);
    Ok(res)
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
    let class_rooms: Vec<class_room::Classroom> = sqlx::query_as("select * from class_rooms where school = $1")
        .bind(&school_auth.school.id)
        .fetch_all(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&class_rooms)?);
    Ok(res)
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
    if let Some(n) = numbers.0
    {
        loop {
            if n.iter().all(|nn| nn != &s) {
                unused_numbers.push(s);
            }
            if unused_numbers.len() >= 10 {
                break;
            }
            s += 1;
        }
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
        let students: Vec<Student> = sqlx::query_as(r#"select id, LEFT(first_name, 3) as "first_name", last_name, school, school_number from students where school = $1"#)
            .bind(&school_auth.school.id)
            .fetch_all(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&students)?);
        Ok(res)
    }
    else {
        Ok(res)
    }
}
pub async fn get_library(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let school_auth: &SchoolAuth = req.ext().unwrap();
    let u = req.user().await?;
    let l = school_auth.school.get_library(&req).await?;
    if school_auth.role < 3 || u.id == l.manager {
        res.set_body(Body::from_json(&l)?);
        Ok(res)
    } else {
        Ok(res)
    }
}
pub async fn get_books(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let library_id = req.param("library_id").unwrap().parse::<i32>().unwrap();
    use sqlx::prelude::PgQueryAs;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 7 {
        let books: Vec<library::Book> = sqlx::query_as(r#"select * from books inner join libraries on books.library = libraries.id where libraries.school = $1 and libraries.id = $2"#)
            .bind(&school_auth.school.id)
            .bind(&library_id)
            .fetch_all(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&books)?);
        Ok(res)
    } else {
        Ok(res)
    }
}
pub async fn books(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    //let library_id = req.param("library_id").unwrap().parse::<i32>().unwrap();
    let book = req.body_json::<library::NewBook>().await?;
    use sqlx::prelude::PgQueryAs;
    let school_auth: &SchoolAuth = req.ext().unwrap();

    if school_auth.role < 7 {
        let user = req.user().await.unwrap();
        let lb: library::Library = sqlx::query_as(r#"select * from libraries where school = $1 and manager = $2"#)
            .bind(&school_auth.school.id)
            .bind(&user.id)
            .fetch_one(&req.state().db_pool).await?;
        if book.barkod >= lb.barkod_min && book.barkod <= lb.barkod_max{
            let b: library::Book = sqlx::query_as(r#"insert into books(library, name, writer, piece, barkod) values($1, $2, $3, $4, $5)
                    returning id, library, name, writer, piece, barkod"#)
                .bind(&lb.id)
                .bind(&book.name).bind(&book.writer).bind(&book.piece).bind(&book.barkod).bind(&lb.barkod_min).bind(&lb.barkod_max)
                .fetch_one(&req.state().db_pool).await?;
            res.set_body(Body::from_json(&b)?);
        }
        Ok(res)
    } else {
        Ok(res)
    }
}
pub async fn library(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx::prelude::PgQueryAs;
    let lbrry = req.body_json::<library::NewLibrary>().await?;
    let _user = req.user().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();

    let l = school_auth.school.get_library(&req).await;
    match l{
        Ok(_) => {
            Ok(res)
        }
        Err(_) => {
            if school_auth.role < 3 {
                let _ = sqlx::query(r#"select * from school_users where school_id = $1 and user_id = $2) "#)
                    .bind(&school_auth.school.id)
                    .bind(&lbrry.manager)
                    .execute(&req.state().db_pool).await?;
                let l2: library::NewLibrary = sqlx::query_as(r#"insert into libraries(school, manager, barkod_min, barkod_max, student)
                                values($1, $2, $3, $4, $5) returning id, manager, school, barkod_min, barkod_max, student"#)
                    .bind(&school_auth.school.id)
                    .bind(&lbrry.manager).bind(lbrry.barkod_min).bind(lbrry.barkod_max).bind(lbrry.student)
                    .fetch_one(&req.state().db_pool).await?;
                res.set_body(Body::from_json(&l2)?);
                Ok(res)
            } else {
                Ok(res)
            }
        }
    }
}
pub async fn patch_library(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx::prelude::PgQueryAs;
    let lbrry = req.body_json::<library::NewLibrary>().await?;
    let _user = req.user().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    let l: sqlx::Result<library::Library> = sqlx::query_as(r#"select * from libraries where school = $1"#)
        .bind(&school_auth.school.id)
        .fetch_one(&req.state().db_pool).await;
    match l {
        Ok(_) => {
            if school_auth.role < 3 {
                let _ = sqlx::query(r#"select * from school_users where school_id = $1 and user_id = $2 "#)
                    .bind(&school_auth.school.id)
                    .bind(&lbrry.manager)
                    .execute(&req.state().db_pool).await?;
                let l2: library::NewLibrary = sqlx::query_as(r#"update libraries set manager= $1, barkod_min = $2, barkod_max = $3, student = $4 where school = $5
                                returning id, manager, school, barkod_min, barkod_max, student"#)
                    .bind(&lbrry.manager).bind(lbrry.barkod_min).bind(lbrry.barkod_max).bind(lbrry.student).bind(&school_auth.school.id)
                    .fetch_one(&req.state().db_pool).await?;
                res.set_body(Body::from_json(&l2)?);
                Ok(res)
            } else {
                Ok(res)
            }
        }
        Err(_) => {
            Ok(res)
        }
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

