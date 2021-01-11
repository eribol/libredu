use tide::{Request, Body, StatusCode};
use crate::AppState;
//use crate::model::student::{NewStudent, Student};
use crate::request::SchoolAuth;

pub async fn del_student(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    let number = req.param("student_id").unwrap().parse::<i32>().unwrap();
    let school_auth: &SchoolAuth = req.ext().unwrap();
    //use sqlx::prelude::PgQueryAs;
    if school_auth.role < 5{
        let _ = sqlx::query("delete from students where id = $1 and school = $2")
            .bind(&number)
            .bind(&school_auth.school.id)
            .execute(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&number)?);
    }
    Ok(res)
}