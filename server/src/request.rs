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
use crate::model::teacher::Teacher;


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
    async fn user(&self)->sqlx_core::Result<AuthUser>;
    async fn is_auth(&self)-> bool;
    async fn get_school(&self)-> sqlx_core::Result<SchoolDetail>;
    async fn get_schools(&self)-> sqlx_core::Result<Vec<(i32, SchoolDetail)>>;
    async fn get_school_auth(&self)-> i32;
    async fn get_group(&self)-> sqlx_core::Result<ClassGroups>;
    async fn get_class(&self)-> Option<Class>;
    // async fn get_group_auth(&self)-> i32;
    async fn get_teacher(&self) -> sqlx_core::Result<crate::model::teacher::Teacher>;
    async fn login(&self, uname: &str, pas: &str)->  bool;
}

#[tide::utils::async_trait]
impl Auth for Request<AppState>{
    async fn user(&self)->sqlx_core::Result<AuthUser> {
        use sqlx_core::postgres::PgQueryAs;
        let user_id = self.cookie("libredu-user");
        if let Some(id) = user_id{
            if let Ok(i) = id.value().parse::<i32>(){
                let user: AuthUser = sqlx::query_as("SELECT * FROM users WHERE id = $1")
                    .bind(i)
                    //.bind(hash(&f.password))
                    .fetch_one(&self.state().db_pool).await?;
                return Ok(user)
            }
        }
        Err(sqlx_core::Error::ColumnNotFound(Box::from("Kullanıcı bulunamadı")))
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
    async fn get_school(&self)-> sqlx_core::Result<SchoolDetail> {
        let school_id = self.param("school");
        if let Ok(id) = school_id{
            if let Ok(i) = id.parse::<i32>(){
                return SchoolDetail::get(&self, i).await
            }
        }
        Err(sqlx_core::Error::ColumnNotFound(Box::from("Kurum bulunamadı")))
    }
    async fn get_schools(&self)-> sqlx_core::Result<Vec<(i32, SchoolDetail)>> {
        use sqlx::Cursor;
        let user = self.user().await?;
        let mut s: Vec<(i32, SchoolDetail)> = vec![];
        let mut query = sqlx::query("SELECT school.id, school.name, school.manager, school.school_type, city.pk, city.name, town.pk, town.name, school_users.role \
                    FROM school inner join town on school.town = town.pk inner join city on town.city = city.pk \
                    inner join school_users on school_users.school_id = school.id WHERE school_users.user_id = $1")
            .bind(&user.id)
            .fetch(&self.state().db_pool);
        while let Some(row) = query.next().await? {
            use sqlx::Row;
            let school = SchoolDetail::get(&self, row.get(0)).await?;
            s.push((row.get(8), school))
        }
        Ok(s)
    }
    async fn get_school_auth(&self) -> i32 {
        let school = self.get_school().await;
        if let Ok(s) = school {
            use sqlx::prelude::PgQueryAs;
            let user_id = self.user().await;
            match user_id {
                Err(_) => {
                    return 9
                }
                Ok(user) => {
                    if user.is_admin {
                        return 1
                    }
                    else {
                        let auth: sqlx::Result<Role> = sqlx::query_as("SELECT *
                                FROM school_users WHERE school_id = $1 and user_id = $2")
                            .bind(&s.id)
                            .bind(&user.id)
                            .fetch_one(&self.state().db_pool).await;
                        match auth {
                            Ok(a) => {
                                return a.role
                            }
                            Err(_) => {
                                let auth2: sqlx::Result<School> = sqlx::query_as("SELECT *
                                            FROM school WHERE id = $1 and manager = $2")
                                    .bind(&s.id)
                                    .bind(&user.id)
                                    .fetch_one(&self.state().db_pool).await;
                                match auth2 {
                                    Ok(_) => {
                                        return 1
                                    }
                                    Err(_) => {
                                        return 9
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        9
    }
    async fn get_group(&self)-> sqlx_core::Result<ClassGroups> {
        let group_id = self.param("group_id").ok().expect("Kurum bulunamadı").parse::<i32>().expect("Kurum Bulunamadı");
        let school = self.get_school().await?;
        school.get_group(&self, group_id).await
            /*
        if let Ok(group_id) = self.param("group_id").ok()?.parse::<i32>() {
            use sqlx_core::postgres::PgQueryAs;
            let school_id = self.param("school").ok()?.parse::<i32>().unwrap();
            let group: ClassGroups = sqlx::query_as("SELECT *
            FROM class_groups WHERE id = $1 and school = $2")
                .bind(&group_id)
                .bind(&school_id)
                .fetch_one(&self.state().db_pool).await?;
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
        }*/
    }
    async fn get_class(&self)-> Option<Class> {
        if let Ok(group) = self.get_group().await {
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
    /*
    async fn get_group_auth(&self) -> i32 {
        //let path = &self.url().path()[1..];
        let school = self.get_group().await;
        if let Ok(s) = school {
            use sqlx::prelude::PgQueryAs;
            let user_id = self.cookie("libredu-user");
            match user_id {
                None => {
                    9
                }
                Some(id) => {
                    let auth: sqlx::Result<Role> = sqlx::query_as("SELECT *
                                FROM school_users WHERE school_id = $1 and user_id = $2")
                        .bind(&s.id)
                        .bind(&id.value().parse::<i32>().unwrap())
                        .fetch_one(&self.state().db_pool).await;
                    match auth {
                        Ok(a) => { a.role }
                        Err(_) => { 9 }
                    }
                }
            }
        }
        9
    }
     */
    async fn get_teacher(&self) -> sqlx_core::Result<crate::model::teacher::Teacher> {
        use sqlx::Cursor;
        use sqlx::Row;
        let teacher_id: i32 = self.param("teacher_id").expect("Id bulunamadı").parse().expect("Id bulunamadı");
        let school_id: i32 = self.param("school").expect("Id bulunamadı").parse().expect("Id bulunamadı");
        Teacher::get(&self, school_id, teacher_id).await
    }
    async fn login(&self, uname: &str, pas: &str)-> bool{
        verify(pas, uname).unwrap()
    }

}