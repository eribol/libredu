use std::io::ErrorKind;

use crate::{tokio::sync::RwLock, user::signin, send_mail::send_mail};
use moon::{Lazy, *, futures::TryStreamExt};
use redis::{self, RedisError};
use shared::{signin::SigninForm, DownMsg};

use self::sql::POSTGRES;
pub mod school;
pub mod sql;
pub mod forget_password;
pub mod reset_password;
pub mod sessions;
pub mod admin;

static REDISDB: Lazy<RwLock<redis::Client>> =
    Lazy::new(|| RwLock::new(redis::Client::open("redis://127.0.0.1:6379/").unwrap()));

async fn get_connection() -> redis::RedisResult<redis::Connection> {
    let client = REDISDB.read().await;
    client.get_connection()
}
pub async fn get_user(
    auth: &str
) -> redis::RedisResult<i32> {
    let mut con = get_connection().await?;
    let user: i32 = redis::cmd("hget")
        .arg("sessions")
        .arg(auth)
        .query(&mut con)?;
    if user == auth.split(':').next().unwrap().parse::<i32>().unwrap(){
        Ok(user)
    }
    else{
        Err(RedisError::from(std::io::Error::new(ErrorKind::NotFound, "E")))
    }
}

pub async fn set_user(id: i32, auth_token: &AuthToken) -> redis::RedisResult<()> {
    let mut con = get_connection().await.unwrap();
    println!("set user oluyor");
    let _user: i32 = redis::cmd("hset")
        .arg("sessions")
        .arg(&auth_token.clone().into_string())
        .arg(id)
        .query(&mut con)?;
    println!("user set user oldu");
    set_session_pg(id, &auth_token.clone().into_string()).await.expect("pg hatası");
    println!("pg user set user oldu");
    Ok(())
}
pub async fn set_session_pg(id: i32, token: &String) -> sqlx::Result<()> {
    let db = POSTGRES.read().await;  
    let mut _user = sqlx::query(r#"insert into session(user_id, key) values($1, $2)"#)
        .bind(id)
        .bind(token)
        .fetch(&*db);
    if let Some(_) = _user.try_next().await.unwrap(){
        println!("noluyor lan");
    }
    Ok(())
}

pub async fn _get_sessions_pg(id: i32, _token: &String) -> redis::RedisResult<()> {
    let db = POSTGRES.read().await; 
    let mut user = sqlx::query(r#"delete from session where user_id = $1 returning key"#)
        .bind(id)
        .fetch(&*db);
    while let Some(_key) = user.try_next().await.unwrap(){
        //del_user(id, key.try_get("key").unwrap()).await;
    }
    Ok(())
}
pub async fn register(user: SigninForm, auth_token: &AuthToken) -> DownMsg{
    let client = REDISDB.read().await;
    let mut con = client.get_connection().unwrap();
    let _user: i32 = redis::cmd("hset")
        .arg(auth_token.clone().into_string())
        .arg(&user.email)
        .arg(serde_json::to_string(&user).unwrap())
        .query(&mut con).unwrap_or(0);
    redis::cmd("expire")
        .arg(auth_token.clone().into_string())
        .arg(60*60);
    let d = dotenvy::var("DOMAIN_NAME").unwrap();
    
    let html = create_html(d, user.email.clone(),  auth_token.clone().into_string().trim().to_string());
    
    send_mail(user.email, html, String::from("Üyelik Onayla")).await
}

fn create_html(d: String, email: String, token: String)->String{
    let addr = format!(r"{d}/register/{token}/{email}");
    let r = format!(r"<!DOCTYPE html>
    <html>
    <body><p>Hesabınızı aktifleştirmek için linke <a href={addr}>tıklayın</a></p>
    
    </body>
    </html>
    ");
    println!("{r:?}");
    r
}
pub async fn get_register(auth_token: String, email: String) -> DownMsg {
    let client = REDISDB.read().await;
    let mut con = client.get_connection().unwrap();
    let _user: String = redis::cmd("hget")
        .arg(auth_token)
        .arg(&email)
        .query(&mut con).unwrap();
    let user: SigninForm = serde_json::from_str(&_user).unwrap();
    let user2: DownMsg = signin(user.clone()).await;
    user2
}
pub async fn del_user(id: i32, auth: String) -> redis::RedisResult<()> {
    let client = REDISDB.read().await;
    let mut con = client.get_connection()?;
    let user: i32 = redis::cmd("hget")
        .arg("sessions")
        .arg(auth.clone())
        .query(&mut con)?;
    if user == id{
        let _user: i32 = redis::cmd("hdel")
        .arg("sessions")
        .arg(auth)
        .query(&mut con)?;
    }
    Ok(())
}
