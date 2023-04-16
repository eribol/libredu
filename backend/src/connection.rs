use std::io::ErrorKind;

use crate::{tokio::sync::RwLock, user::signin, send_mail::send_mail};
use moon::{Lazy, *};
use redis::{self, RedisError};
use shared::{User, signin::SigninForm, DownMsg};
pub mod school;
pub mod sql;

static REDISDB: Lazy<RwLock<redis::Client>> =
    Lazy::new(|| RwLock::new(redis::Client::open("redis://127.0.0.1:6379/").unwrap()));

async fn get_connection() -> redis::RedisResult<redis::Connection> {
    let client = REDISDB.write().await;
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
    println!("aaa {:?}", &user);
    if user == auth.split(":").nth(0).unwrap().parse::<i32>().unwrap(){
        return Ok(user)
    }
    else{
        return Err(RedisError::from(std::io::Error::new(ErrorKind::NotFound, "E")))
    }
}

pub async fn set_user(id: i32, auth_token: &AuthToken) -> redis::RedisResult<()> {
    let client = REDISDB.write().await;
    let mut con = client.get_connection()?;
    let _user: i32 = redis::cmd("hset")
        .arg("sessions")
        .arg(auth_token.clone().into_string())
        .arg(id)
        .query(&mut con)?;
    Ok(())
}

pub async fn register(user: SigninForm, auth_token: &AuthToken) -> redis::RedisResult<()> {
    let client = REDISDB.write().await;
    let mut con = client.get_connection()?;
    let _user: i32 = redis::cmd("hset")
        .arg(auth_token.clone().into_string())
        .arg(&user.email)
        .arg(serde_json::to_string(&user).unwrap())
        .query(&mut con)?;
    redis::cmd("expire")
        .arg(auth_token.clone().into_string())
        .arg(60*60);
    let d = dotenvy::var("DOMAIN_NAME").unwrap();
    
    let html = create_html(d, user.email.clone(),  auth_token.clone().into_string().trim().to_string());
    send_mail(user.email, html);
    Ok(())
}

fn create_html(d: String, email: String, token: String)->String{
    let r = format!(r#"<!DOCTYPE html>
    <html>
    <body><p>Hesabınızı aktifleştirmek için linke <a href={d:?}/register/{token:?}/{email:?}>tıklayın</a></p>
    
    </body>
    </html>
    "#);
    r
}
pub async fn get_register(auth_token: String, email: String) -> DownMsg {
    let client = REDISDB.write().await;
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
    let client = REDISDB.write().await;
    let mut con = client.get_connection()?;
    let _user = redis::cmd("hdel")
        .arg("sessions")
        .arg(id)
        .arg(auth)
        .query(&mut con)?;
    Ok(())
}
