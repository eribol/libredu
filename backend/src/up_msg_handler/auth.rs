use crate::tokio::sync::RwLock;
use moon::{Lazy, *};
use redis;
use sqlx::{PgPool, Pool, Postgres};

pub static POSTGRES: Lazy<RwLock<PgPool>> = Lazy::new(|| {
    RwLock::new(
        Pool::<Postgres>::connect_lazy(
            &dotenvy::var("DATABASE_URL").expect("Database_url must be set"),
        )
        .expect("Failed to connect db"),
    )
});

static REDISDB: Lazy<RwLock<redis::Client>> =
    Lazy::new(|| RwLock::new(redis::Client::open("redis://127.0.0.1:6379/").unwrap()));
pub async fn get_user<'a, U: Deserialize<'a> + redis::FromRedisValue>(
    auth: &'a str,
) -> redis::RedisResult<U> {
    let mut con = REDISDB
        .read()
        .await
        .get_connection()
        .expect("No redis Connection");
    let user: U = redis::cmd("hget")
        .arg("sessions")
        .arg(auth)
        .query(&mut con)?;
    Ok(user)
}

pub async fn _set_user(id: i32, auth_token: &AuthToken) -> redis::RedisResult<()> {
    let client = REDISDB.write().await;
    let mut con = client.get_connection()?;
    let _user: i32 = redis::cmd("hset")
        .arg("sessions")
        .arg(auth_token.clone().into_string())
        .arg(id)
        .query(&mut con)?;
    Ok(())
}

pub async fn _del_user(id: i32, auth: String) -> redis::RedisResult<()> {
    let client = REDISDB.write().await;
    let mut con = client.get_connection()?;
    let _user = redis::cmd("hdel")
        .arg("sessions")
        .arg(id)
        .arg(auth)
        .query(&mut con)?;
    Ok(())
}

pub async fn auth(auth_token: Option<AuthToken>) -> Option<i32> {
    match auth_token {
        Some(auth) => {
            let user_id: redis::RedisResult<i32> = get_user(&auth.into_string()).await;
            match user_id {
                Ok(id) => Some(id),
                Err(_e) => None,
            }
        }
        None => None,
    }
}
