use crate::connection::{self, set_user};
use bcrypt::{hash, verify};
use chrono;
use moon::*;
use shared::{DownMsg, User};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(crate = "serde")]
pub struct LoginUser {
    pub id: i32,
    pub first_name: String,
    password: String,
}

pub async fn login(email: String, password: String) -> sqlx::Result<LoginUser> {
    let db = connection::sql::POSTGRES.read().await;
    let user: sqlx::Result<LoginUser> =
        sqlx::query_as(r#"select id, first_name, password from users where email = $1"#)
            .bind(&email)
            //.bind(verify(password, ))
            .fetch_one(&*db)
            .await;
    match user {
        Ok(u) => {
            if verify(&password, &u.password).unwrap() {
                Ok(u)
            } else {
                Err(sqlx::Error::RowNotFound)
            }
        }
        Err(_e) => Err(sqlx::Error::RowNotFound),
    }
    //user
}

pub async fn signin2(form: shared::signin::SigninForm){
    
}
pub async fn signin(form: shared::signin::SigninForm) -> DownMsg {
    let db = connection::sql::POSTGRES.read().await;
    if form.is_valid().is_ok(){
        let user: sqlx::Result<LoginUser> = sqlx::query_as(
            r#"insert into users(first_name,last_name,
            email, password, date_join) values($1, $2, $3, $4, $5) returning id, first_name,last_name"#,
        )
        .bind(&form.first_name)
        .bind(&form.last_name)
        .bind(&form.email)
        .bind(hash(&form.password, 10).unwrap())
        .bind(&chrono::Utc::now())
        .fetch_one(&*db)
        .await;
        match user{
            Ok(u) => {
                let token = EntityId::new();
                let token = AuthToken::new(
                    format!("{}:{}", u.id, token)
                );
                let user = User{id: u.id, first_name: u.first_name, auth_token: token.clone()};
                let _ = set_user(u.id, &token).await;
                return  DownMsg::Registered(user)
            },
            Err(_e) => return DownMsg::ResgiterErrors
        }
    }
    DownMsg::ResgiterErrors
}
