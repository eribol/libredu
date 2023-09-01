use crate::connection::{self, set_user};
use bcrypt::{hash, verify};

use moon::*;
use shared::{DownMsg, User};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(crate = "serde")]
pub struct LoginUser {
    pub id: i32,
    pub first_name: String,
    password: String,
    pub is_active: bool,
    pub is_admin: bool
}

pub async fn login(email: String, password: String) -> sqlx::Result<LoginUser> {
    let db = connection::sql::POSTGRES.read().await;
    let user: sqlx::Result<LoginUser> =
        sqlx::query_as(r#"select id, first_name, password, is_active, is_admin from users where email = $1 and email is not null"#)
            .bind(&email)
            //.bind(verify(password, ))
            .fetch_one(&*db)
            .await;
    match user {
        Ok(u) => {
            if verify(&password, &u.password).unwrap() {
                let _ = sqlx::query(r#"update users set last_login = $1 where email = $2"#)
                .bind(Utc::now().naive_utc())
                .bind(&email)
                //.bind(verify(password, ))
                .execute(&*db).await;
                Ok(u)
            } else {
                Err(sqlx::Error::RowNotFound)
            }
        }
        Err(_e) => Err(sqlx::Error::RowNotFound),
    }
    //user
}

pub async fn is_user_exist(email: &String)-> sqlx::Result<LoginUser>{
    let db = connection::sql::POSTGRES.read().await;
    let user: sqlx::Result<LoginUser> =sqlx::query_as(r#"
        select id, first_name, password, is_active, is_admin from users where email = $1
    "#).bind(email).fetch_one(&*db).await;
    user
}
pub async fn signin(form: shared::signin::SigninForm) -> DownMsg {
    let db = connection::sql::POSTGRES.read().await;
    if form.is_valid().is_ok(){
        let user: sqlx::Result<LoginUser> = sqlx::query_as(
            r#"insert into users(first_name,last_name,
            email, password, date_join) values($1, $2, $3, $4, $5) returning id, first_name,last_name,is_active, is_admin"#,
        )
        .bind(&form.first_name)
        .bind(&form.last_name)
        .bind(&form.email)
        .bind(hash(&form.password, 10).unwrap())
        .bind(chrono::Utc::now())
        .fetch_one(&*db)
        .await;
        match user{
            Ok(u) => {
                let token = EntityId::new();
                let token = AuthToken::new(
                    format!("{}:{}", u.id, token)
                );
                let user = User{id: u.id, first_name: u.first_name, auth_token: token.clone(), is_admin: u.is_admin, is_active: u.is_active};
                let _ = set_user(u.id, &token).await;
                return  DownMsg::Registered(user)
            },
            Err(_e) => return DownMsg::ResgiterErrors
        }
    }
    DownMsg::ResgiterErrors
}

pub async fn get_user_with_id(id: i32)-> sqlx::Result<LoginUser>{
    let db = connection::sql::POSTGRES.read().await;
    let user: sqlx::Result<LoginUser> =sqlx::query_as(r#"
        select id, first_name, password, is_active, is_admin from users where id = $1
    "#).bind(id).fetch_one(&*db).await;
    user
}