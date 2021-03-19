use serde::*;
use crate::model::city::{City, Town};
use crate::AppState;
use crate::model::{class, subject, group, teacher, library};
use crate::model::activity::Activity;


#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct School{
    pub(crate) id: i32,
    name: String,
    pub(crate) manager: i32
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct SchoolDetail {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub manager: i32,
    pub school_type: i32,
    pub tel: Option<String>,
    pub location: Option<String>,
    pub city: City,
    pub town: Town
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct SchoolTeacher {
    pub(crate) school_id: i32,
    pub(crate) user_id: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NewSchool {
    pub(crate) name: String,
    pub school_type: i32,
    pub city: i32,
    pub town: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, sqlx::FromRow)]
pub struct SchoolType {
    pub(crate) name: String,
    pub(crate) id: i32
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateSchoolForm{
    pub(crate) name: String,
    pub(crate) tel: Option<String>,
    pub(crate) location: Option<String>
}

impl NewSchool{
    pub async fn add(&self, req: &mut tide::Request<AppState>, id: i32) -> sqlx_core::Result<School> {
        use sqlx::prelude::PgQueryAs;
        let add_school: sqlx::Result<School> = sqlx::query_as
            (r#"INSERT into school (name, town, school_type, manager) values($1, $2, $3, $4) returning id, name, manager"#)
            .bind(&self.name)
            .bind(&self.town)
            .bind(&self.school_type)
            .bind(&id)
            .fetch_one(&req.state().db_pool).await;
        add_school
    }
}

impl SchoolDetail{
    pub async fn get_teachers(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<teacher::Teacher>>{
        use sqlx::Cursor;
        use sqlx::Row;
        let mut tchrs = sqlx::query("SELECT users.id, users.first_name, users.last_name, roles.id, roles.name \
                        FROM school_users inner join users on school_users.user_id = users.id inner join roles on school_users.role = roles.id \
                        WHERE school_users.school_id = $1 and school_users.role <= 5 order by roles.id, users.first_name")
            .bind(&self.id)
            .fetch(&req.state().db_pool);
        let mut teachers: Vec<teacher::Teacher> = vec![];
        while let Some(row) = tchrs.next().await? {
            let teacher = teacher::Teacher {
                id: row.get(0),
                first_name: row.get(1),
                last_name: row.get(2),
                role_id: row.get(3),
                role_name: row.get(4)
            };
            teachers.push(teacher);
        }
        Ok(teachers)
    }
    pub async fn get_classes(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<class::Class>>{
        use sqlx::prelude::PgQueryAs;
        let classes: Vec<class::Class> = sqlx::query_as("SELECT * FROM classes WHERE school = $1 order by kademe, sube")
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(classes)
    }
    pub async fn get_subjects(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<subject::Subject>>{
        use sqlx::prelude::PgQueryAs;
        let subjects: Vec<subject::Subject> = sqlx::query_as("SELECT * FROM subjects WHERE school = $1 order by optional, name, kademe")
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(subjects)
    }
    pub async fn get_groups(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<Vec<group::ClassGroups>>{
        use sqlx::prelude::PgQueryAs;
        let groups: Vec<group::ClassGroups> = sqlx::query_as("select * from class_groups where school = $1")
            .bind(&self.id)
            .fetch_all(&req.state().db_pool).await?;
        Ok(groups)
    }
    pub async fn get_library(&self, req: &tide::Request<AppState>) -> sqlx_core::Result<library::Library>{
        use sqlx::prelude::PgQueryAs;
        let lbrry: library::Library  = sqlx::query_as(r#"select * from libraries where school = $1"#)
            .bind(&self.id)
            .fetch_one(&req.state().db_pool).await?;
        Ok(lbrry)
    }
    pub async fn get_activities(&self, req: &tide::Request<AppState>, group:&i32 ) -> sqlx_core::Result<Activity>{
        let act = Activity{
            id: 0,
            subject: 0,
            teacher: 0,
            hour: 0,
            split: false,
            classes: vec![]
        };
        Ok(act)
    }
}