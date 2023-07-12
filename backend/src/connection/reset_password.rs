use bcrypt::hash;
use moon::*;
use shared::{DownMsg, models::users::ResetForm, User};
use crate::connection::{self, set_user};
use crate::{send_mail::{self, send_mail}, user::LoginUser};
use super::REDISDB;

pub async fn reset_password(form: ResetForm)-> DownMsg{
    if get_token(&form.email, &form.token).await{
        println!("reset geldi");
        let db = super::sql::POSTGRES.read().await;
        let user: sqlx::Result<LoginUser> = sqlx::query_as(r#"update users set password = $2 where email = $1 returning id, first_name, password"#)
            .bind(&form.email)
            .bind(hash(&form.password, 10).unwrap())
            .fetch_one(&*db)
            .await;
        match user{
            Ok(u)=>{
                println!("user doğru");
                let token = format!("{}:{}", u.id, EntityId::new());
                let user = User{
                    id: u.id,
                    first_name: u.first_name,
                    auth_token: AuthToken::new(token)
                };
                set_user(user.id, &user.auth_token).await;
                println!("set user oldu");
                return DownMsg::LoggedIn(user)
            },
            Err(e)=>{
                return DownMsg::LoginError("Not found user".to_string())
            }
        }
    }
    DownMsg::ResetPassword
}
async fn get_token(email: &String, auth_token: &String)-> bool{
    let client = REDISDB.write().await;
    let mut con = client.get_connection().unwrap();
    let mail: String  = redis::cmd("get")
        .arg(auth_token.clone())
        .query(&mut con).unwrap();
    email == &mail
}

async fn set_token(email: &String, auth_token: &String)-> bool{
    let client = REDISDB.write().await;
    let mut con = client.get_connection().unwrap();
    let mail: String  = redis::cmd("set")
        .arg(auth_token.clone())
        .query(&mut con).unwrap();
    email == &mail
}

pub async fn del_reset(id: i32, auth: String) -> redis::RedisResult<()> {
    let client = REDISDB.write().await;
    let mut con = client.get_connection()?;
    let _user = redis::cmd("hdel")
        .arg("reset_passwords")
        .arg(auth)
        .query(&mut con)?;
    Ok(())
}