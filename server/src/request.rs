use serde::*;
use tide::{Request};
use crate::AppState;
use crate::model::user::AuthUser;
use bcrypt::verify;
//use async_trait::async_trait;
use crate::model::school::{School, SchoolDetail};
use crate::model::city::{City, Town};
use crate::model::group::ClassGroups;
use crate::model::class::Class;
use anyhow::anyhow;

#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Role{
    role: i32
}
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct SchoolAuth{
    pub school: SchoolDetail,
    pub(crate) role: i32,
}
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct SchoolMenu{
    link: String,
    name: String,
    id: i32
}
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct GroupMenu{
    link: String,
    name: String,
    id: i32
}
#[tide::utils::async_trait]
pub trait Auth{
    async fn user(&self)->anyhow::Result<AuthUser>;
    async fn is_auth(&self)-> bool;
    async fn get_school(&self)-> Option<SchoolDetail>;
    async fn get_school_auth(&self)-> i32;
    async fn get_group(&self)-> Option<ClassGroups>;
    async fn get_class(&self)-> Option<Class>;
    async fn get_group_auth(&self)-> i32;
    async fn get_teacher(&self) -> sqlx_core::Result<crate::model::teacher::Teacher>;
    async fn login(&self, uname: &str, pas: &str)->  bool;
}

#[tide::utils::async_trait]
impl Auth for Request<AppState>{
    async fn user(&self)->anyhow::Result<AuthUser> {
        use sqlx_core::postgres::PgQueryAs;
        let user_id = self.cookie("libredu-user").ok_or_else(|| anyhow!("No session"))?;
        let user: AuthUser = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(user_id.value().parse::<i32>().expect("Id bulunamadı"))
            //.bind(hash(&f.password))
            .fetch_one(&self.state().db_pool).await?;
        Ok(user)
    }
    async fn is_auth(&self)-> bool{
        let user_id = self.cookie("libredu-user");
        match user_id{
            Some(u)=>{
                let session_key = self.cookie("libredu-uuid");
                match session_key{
                    Some(key)=>{
                        //use sqlx_core::postgres::PgQueryAs;
                        let session = sqlx::query!(r#"select * from session where user_id = $1 and key = $2;"#, u.value().parse::<i32>().unwrap(), key.value())
                            //.bind(hash(&f.password))
                            .fetch_one(&self.state().db_pool).await;
                        match session{
                            Ok(_)=>{
                                true
                            },
                            Err(_)=>{
                                false
                            }
                        }
                    },
                    None=>{
                        false
                    }
                }
            },
            None=>{
                false
            }
        }
    }
    async fn get_school(&self)-> Option<SchoolDetail> {
        if let Ok(school_id) = self.param("school").ok()?.parse::<i32>() {
            use sqlx_core::cursor::Cursor;
            use sqlx_core::row::Row;
            let mut s = SchoolDetail::default();
            let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, school.tel, school.location, city.pk, city.name, town.pk, town.name \
            FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk WHERE school.id = $1")
                .bind(&school_id)
                .fetch(&self.state().db_pool);
            while let Some(row) = query.next().await.unwrap() {
                s = SchoolDetail {
                    id: row.get(0),
                    name: row.get(1),
                    manager: row.get(2),
                    school_type: row.get(3),
                    tel: row.get(4),
                    location: row.get(5),
                    city: City { pk: row.get(6), name: row.get(7) },
                    town: Town {
                        city: row.get(6),
                        pk: row.get(8),
                        name: row.get(9)
                    }
                }
            }
            if s.id > 0 {
                Some(s)
            } else {
                None
            }
        } else {
            None
        }
    }
    async fn get_school_auth(&self) -> i32 {
        let school = self.get_school().await;
        match school{
            Some(s) => {
                use sqlx::prelude::PgQueryAs;
                let user_id = self.user().await;
                match user_id{
                    Err(_) => {
                        9
                    }
                    Ok(user) => {
                        if user.is_admin{
                            1
                        }
                        else {
                            let auth: sqlx::Result<Role> = sqlx::query_as("SELECT *
                                FROM school_users WHERE school_id = $1 and user_id = $2")
                                .bind(&s.id)
                                .bind(&user.id)
                                .fetch_one(&self.state().db_pool).await;
                            match auth{
                                Ok(a) => {
                                    a.role
                                }
                                Err(_) => {
                                    let auth2: sqlx::Result<School> = sqlx::query_as("SELECT *
                                            FROM school WHERE id = $1 and manager = $2")
                                        .bind(&s.id)
                                        .bind(&user.id)
                                        .fetch_one(&self.state().db_pool).await;
                                    match auth2{
                                        Ok(_) => {
                                            1
                                        }
                                        Err(_) => {
                                            9
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => {
                9
            }
        }

    }
    async fn get_group(&self)-> Option<ClassGroups> {
        if let Ok(group_id) = self.param("group_id").ok()?.parse::<i32>() {
            use sqlx_core::postgres::PgQueryAs;
            let school_id = self.param("school").ok()?.parse::<i32>().unwrap();
            let group: sqlx::Result<ClassGroups> = sqlx::query_as("SELECT *
            FROM class_groups WHERE id = $1 and school = $2")
                .bind(&group_id)
                .bind(&school_id)
                .fetch_one(&self.state().db_pool).await;
            match group {
                Ok(g) => {
                    Some(g)
                }
                Err(_) =>{
                    None
                }
            }
        }
        else {
            None
        }
    }
    async fn get_class(&self)-> Option<Class> {
        if let Some(group) = self.get_group().await {
            use sqlx_core::postgres::PgQueryAs;
            let class_id = self.param("class_id").ok()?.parse::<i32>();
            match class_id{
                Ok(c) => {
                    let class: sqlx::Result<Class> = sqlx::query_as("SELECT *
                            FROM classes WHERE group_id = $1 and id = $2")
                        .bind(&group.id)
                        .bind(&c)
                        .fetch_one(&self.state().db_pool).await;
                    match class {
                        Ok(c) => {
                            Some(c)
                        }
                        Err(_) =>{
                            None
                        }
                    }
                }
                Err(_) => {
                    None
                }
            }

        }
        else {
            None
        }
    }
    async fn get_group_auth(&self) -> i32 {
        //let path = &self.url().path()[1..];
        let school = self.get_group().await;
        match school{
            Some(s) => {
                use sqlx::prelude::PgQueryAs;
                let user_id = self.cookie("libredu-user");
                match user_id{
                    None => {
                        9
                    }
                    Some(id) => {
                        let auth: sqlx::Result<Role> = sqlx::query_as("SELECT *
                                FROM school_users WHERE school_id = $1 and user_id = $2")
                            .bind(&s.id)
                            .bind(&id.value().parse::<i32>().unwrap())
                            .fetch_one(&self.state().db_pool).await;
                        match auth{
                            Ok(a) => {a.role}
                            Err(_) => {9}
                        }
                    }
                }
            }
            None => {
                9
            }
        }

    }
    async fn get_teacher(&self) -> sqlx_core::Result<crate::model::teacher::Teacher> {
        use sqlx::Cursor;
        use sqlx::Row;
        let teacher_id: i32 = self.param("teacher_id").expect("Id bulunamadı").parse().expect("Id bulunamadı");
        let school_id: i32 = self.param("school").expect("Id bulunamadı").parse().expect("Id bulunamadı");
        let mut tchr = sqlx::query("SELECT users.id, users.first_name, users.last_name, roles.id, roles.name, users.is_active, users.email, users.tel \
                        FROM school_users inner join users on school_users.user_id = users.id inner join roles on school_users.role = roles.id \
                        WHERE school_users.school_id = $1 and school_users.role <= 5 and user_id = $2 order by roles.id, users.first_name")
            .bind(&school_id)
            .bind(&teacher_id)
            .fetch(&self.state().db_pool);
        if let Some(row) = tchr.next().await? {
            let teacher = crate::model::teacher::Teacher {
                id: row.get(0),
                first_name: row.get(1),
                last_name: row.get(2),
                role_id: row.get(3),
                role_name: row.get(4),
                is_active: row.get(5),
                email: row.get(6),
                tel: row.get(7)
            };
            return Ok(teacher)
        }
        Err(sqlx_core::Error::ColumnNotFound(Box::from("Öğretmen bulunamadı")))
    }
    async fn login(&self, uname: &str, pas: &str)-> bool{
        verify(pas, uname).unwrap()
    }

}