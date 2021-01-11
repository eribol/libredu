use tide::{Request, Response};
use tide::StatusCode;
use crate::AppState;
use http_types::{Body};
use crate::model::user::AuthUser;
use crate::model::school::School;
use crate::model::group::ClassGroups;
use crate::model::{class, teacher};
use crate::views::ResetPassword;

pub async fn get(req: Request<AppState>)-> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    let user: &AuthUser = req.ext().unwrap();
    let user_id: i32 = req.param("user_id")?.parse()?;
    if user.id == user_id {
        use sqlx_core::postgres::PgQueryAs;
        let get_user: AuthUser = sqlx::query_as(r#"SELECT * from users where id = $1"#)
            .bind(&user.id)
            .fetch_one(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&get_user)?);
        Ok(res)
    } else {
        Ok(res)
    }
}


pub async fn get_schools(req: Request<AppState>)-> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    let user: &AuthUser = req.ext().unwrap();
    use sqlx_core::postgres::PgQueryAs;
    let get_user: Vec<School> = sqlx::query_as(r#"SELECT school.id, school.name, school.manager
        from school inner join school_users on school.id = school_users.school_id where school_users.user_id = $1"#)
        .bind(&user.id)
        .fetch_all(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&get_user)?);
    Ok(res)
}

pub async fn get_timetables(req: Request<AppState>)-> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    let user: &AuthUser = req.ext().unwrap();
    let user_id: i32 = req.param("user_id")?.parse()?;
    if user.id == user_id {
        use sqlx_core::postgres::PgQueryAs;
        let schools: Vec<School> = sqlx::query_as(r#"SELECT school.id, school.name, school.manager from school
                        inner join school_users on school.id = school_users.school_id where school_users.user_id = $1"#)
            .bind(&user.id)
            .fetch_all(&req.state().db_pool).await?;
        let mut timetables: Vec<(School, ClassGroups, Vec<teacher::TeacherTimetable>)> = vec![];
        for s in &schools {
            let groups: Vec<ClassGroups> = sqlx::query_as(r#"SELECT * from class_groups where school = $1"#)
                .bind(&s.id)
                .fetch_all(&req.state().db_pool).await?;
            for group in groups {
                use sqlx_core::cursor::Cursor;
                use sqlx_core::row::Row;
                let mut class = sqlx::query("SELECT class_timetable.id, classes.id, classes.kademe, classes.sube, classes.school, classes.group_id,
                            class_timetable.day_id, class_timetable.hour, subjects.name
                            FROM class_timetable inner join activities on class_timetable.activities = activities.id
                            inner join users on activities.teacher = users.id
                            inner join subjects on activities.subject = subjects.id
                            inner join classes on class_timetable.class_id = classes.id
                            WHERE activities.teacher = $1 and classes.group_id = $2")
                    .bind(&user_id)
                    .bind(&group.id)
                    .fetch(&req.state().db_pool);
                let mut teacher_timetables: Vec<teacher::TeacherTimetable> = Vec::new();
                while let Some(row) = class.next().await? {
                    let teacher_timetable = teacher::TeacherTimetable {
                        id: row.get(0),
                        class_id: vec![class::Class {
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
                timetables.push((s.clone(), group, teacher_timetables));
            }
        }


        res.set_body(Body::from_json(&timetables)?);
        Ok(res)
    } else {
        Ok(res)
    }
}

pub async fn post_reset(mut req: Request<AppState>) -> tide::Result {
    let user: &AuthUser = req.ext().unwrap();
    use sqlx_core::postgres::PgQueryAs;
    let user_id: i32 = req.param("user_id")?.parse()?;
    if user.id == user_id {
        let reset_form: ResetPassword = req.body_json().await?;
        let user: sqlx::Result<AuthUser> = sqlx::query_as("SELECT * FROM users where email = $1 and (tel = $2 or tel = $3) and key = $4")
            .bind(&reset_form.email)
            .bind(&reset_form.tel)
            .bind(&bcrypt::hash(reset_form.tel, 8)?)
            .bind(&reset_form.key)
            .fetch_one(&req.state().db_pool).await;
        match user {
            Ok(_u) => {
                if reset_form.password1 == reset_form.password2 {
                    let mut res = tide::Response::new(StatusCode::Ok);
                    let _user: sqlx::Result<AuthUser> = sqlx::query_as("update users set password = $1 where email = $2")
                        .bind(bcrypt::hash(&reset_form.password1, 10)?)
                        .bind(&reset_form.email)
                        .fetch_one(&req.state().db_pool).await;
                    res.set_body(Body::from_json(&"Şifreniz güncellendi")?);
                    Ok(res)
                } else {
                    let res = tide::Response::new(StatusCode::NotAcceptable);
                    Ok(res)
                }
            }
            Err(_) => {
                let res = tide::Response::new(StatusCode::NotAcceptable);
                Ok(res)
            }
        }
    } else {
        let res = tide::Response::new(StatusCode::NotAcceptable);
        Ok(res)
    }
}