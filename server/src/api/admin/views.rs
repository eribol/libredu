use tide::{Request, Body};
use crate::AppState;
use crate::request::Auth;
use serde::*;
use crate::model::school::SchoolType;
use crate::model::subject::Subject;

pub async fn add_school_type(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(tide::StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    let class = req.body_json::<NewType>().await?;
    let u = req.user().await?;
    if u.is_admin || u.id == 91 {
        let add_type: SchoolType = sqlx::query_as("insert into school_type(name) values($1) returning id, name")
            .bind(&class.name)
            .fetch_one(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&add_type)?);
        Ok(res)
    } else {
        Ok(res)
    }
}

pub async fn add_subject(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(tide::StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    let subject = req.body_json::<NewSubject>().await?;
    let u = req.user().await?;
    if u.is_admin {
        let add_type: Subject = sqlx::query_as("insert into subjects(name, kademe, school_type, optional) values($1, $2, $3, $4) returning id, name, kademe, school_type, optional")
            .bind(&subject.name)
            .bind(&subject.kademe)
            .bind(&subject.school_type)
            .bind(&subject.optional)
            .fetch_one(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&add_type)?);
        Ok(res)
    } else {
        let res = tide::Response::new(tide::StatusCode::Ok);
        Ok(res)
    }
}

pub async fn get_subjects(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(tide::StatusCode::Ok);
    let school_type: i32 = req.param("school_type")?.parse()?;
    use sqlx::prelude::PgQueryAs;
    let subjects: Vec<Subject> = sqlx::query_as("SELECT * from subjects where school_type = $1")
        .bind(&school_type)
        .fetch_all(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&subjects)?);
    Ok(res)
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct NewType{
    name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewSubject{
    school_type: i32,
    name: String,
    kademe: String,
    optional: bool
}