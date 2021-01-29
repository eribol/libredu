use tide::{Response, Request, Body, StatusCode};
use crate::AppState;
use tide::http::Cookie;
use serde::*;
use crate::model::user::{User, AuthUser, SignUser};
use uuid::Uuid;
use lettre::{ClientSecurity, Transport};
use http_types::cookies::SameSite;
use crate::request::Auth;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginForm{
    username: String,
    password: String
}

/*pub async fn get_login(req: Request<AppState>) -> tide::Result {
    let user = req.user().await;
    match user {
        Some(u) => {
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&u)?);
            Ok(res)
        },
        None => {
            let res = Response::new(StatusCode::Unauthorized);
            Ok(res)
        }
    }
}*/

pub async fn login(mut req: Request<AppState>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.remove_cookie(Cookie::named("libredu-user"));
    res.remove_cookie(Cookie::named("libredu-uuid"));
    let form = req.body_json::<LoginForm>().await;
    match form {
        Ok(f) => {
            use bcrypt::{verify, hash};
            use sqlx_core::postgres::PgQueryAs;
            let user2: Result<User, sqlx_core::error::Error> = sqlx::query_as("SELECT * FROM users WHERE email = $1 OR tel = $2")
                .bind(&f.username)
                .bind(&hash(&f.username, 8).unwrap())
                .fetch_one(&req.state().db_pool).await;
            match user2 {
                Ok(u) => {
                    //let mut res = tide::Response::new(StatusCode::Ok);
                    if verify(f.password, &u.password).unwrap() {
                        //res.insert_cookie(Cookie::new("user_id", "1"));
                        let auth_user = AuthUser {
                            id: u.id,
                            first_name: u.first_name,
                            last_name: u.last_name,
                            email: u.email,
                            username: u.username,
                            is_admin: u.is_admin
                        };
                        let u_id = Uuid::new_v4().to_string();
                        let _session = sqlx::query!(r#"INSERT into session (user_id, key) values($1,$2)"#, u.id, &u_id)
                            //.bind(hash(&f.password))
                            .execute(&req.state().db_pool).await?;
                        dotenv::dotenv().expect("Failed to read .env file");
                        use std::env;
                        let mut domain = "127.0.0.1".to_string();
                        match env::var("DOMAIN_NAME") {
                            Ok(_) => domain = "libredu.org".to_string(),
                            Err(_) => {
                                //domain = "127.0.0.1".to_string()
                            }
                        };
                        use time::{Duration};
                        //println!("{:?}", env::var("DOMAIN_NAME").unwrap_or("127.0.0.1".to_string()));
                        let _cookie = Cookie::build("libredu-user", u.id.to_string())
                            .domain(domain.clone())
                            .path("/")
                            .secure(true)
                            .same_site(SameSite::None)
                            .max_age(Duration::days(180))
                            .http_only(true)
                            .finish();
                        res.insert_cookie(_cookie);
                        let _cookie = Cookie::build("libredu-uuid", u_id)
                            .domain(domain)
                            .path("/")
                            .secure(true)
                            .same_site(SameSite::None)
                            .max_age(Duration::days(180))
                            .http_only(true)
                            .finish();
                        res.insert_cookie(_cookie);
                        //_cookie.name("libredu-user")
                        //res.insert_cookie(Cookie::new("libredu-user", u.id.to_string()));
                        //res.insert_cookie(Cookie::new("libredu-uuid", u_id));
                        res.set_body(Body::from_json(&auth_user)?);
                        Ok(res)
                    } else {
                        Ok(res)
                    }
                },
                Err(_e) => {
                    let mut res = tide::Response::new(StatusCode::Unauthorized);
                    res.set_body(Body::from_json(&"Girdiğiniz bilgiler uyuşmuyor")?);
                    Ok(res)
                }
            }
        },
        Err(_e) => {
            let mut res = tide::Response::new(StatusCode::BadRequest);
            res.set_body(Body::from_json(&"Form bilgileri eksik veya yanlış")?);
            Ok(res)
        }
    }
}

pub async fn get_user(req: Request<AppState>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    match req.user().await{
        Some(user) => {
            res.set_body(Body::from_json(&user)?);
        }
        None => {}
    }
    Ok(res)
}

pub async fn signin(mut req: Request<AppState>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    let form = req.body_json::<SignUser>().await;
    match form {
        Ok(f) => {
            use sqlx_core::postgres::PgQueryAs;
            let user2: Result<User, sqlx_core::error::Error> = sqlx::query_as("SELECT * FROM users WHERE email = $1 OR tel = $2")
                .bind(&f.email)
                .bind(bcrypt::hash(&f.tel,8).unwrap())
                //.bind(hash(&f.password))
                .fetch_one(&req.state().db_pool).await;
            match user2 {
                Ok(_u) => {
                    let mut res = tide::Response::new(StatusCode::Ok);
                    res.set_body(Body::from_json(&"e-posta veya telefon kayıtlı")?);
                    Ok(res)
                },
                Err(_e) => {
                    if &f.tel.len() != &10 || !f.tel.parse::<f64>().is_ok(){
                        let mut res = tide::Response::new(StatusCode::BadRequest);
                        res.set_body(Body::from_json(&"Telefon numaranız geçerli değil")?);
                        Ok(res)
                    }
                    else if &f.password1 != &f.password2 || &f.password2.len() < &4{
                        let mut res = tide::Response::new(StatusCode::BadRequest);
                        res.set_body(Body::from_json(&"Şifreler uyuşmuyor veya kısa")?);
                        Ok(res)
                    }
                    else if &f.password1 == &"" || &f.password2 == &"" || &f.email == &"" || &f.last_name == &"" || &f.first_name == &""{
                        let mut res = tide::Response::new(StatusCode::BadRequest);
                        res.set_body(Body::from_json(&"Bilgiler boş geçilemez")?);
                        Ok(res)
                    }
                    else if !f.email.contains("@"){
                        let mut res = tide::Response::new(StatusCode::BadRequest);
                        res.set_body(Body::from_json(&"E-posta bilgisi doğru girilmemiş")?);
                        Ok(res)
                    }
                    else{
                        use lettre::{SendableEmail, Envelope, EmailAddress, SmtpClient};
                        let key = Uuid::new_v4().to_string();
                        let email = SendableEmail::new(
                            Envelope::new(
                                Some(EmailAddress::new("root@libredu.org".to_string()).unwrap()),
                                vec![EmailAddress::new(f.email.clone()).unwrap()],
                            ).unwrap(),
                            "User Key".to_string(),
                            key.clone().into_bytes(),
                        );

                        let mut mailer =
                            SmtpClient::new(("127.0.0.1", 25), ClientSecurity::None).unwrap().transport();
                        // Send the email
                        let result = mailer.send(email);
                        match result{
                            Ok(_r)=> {
                                let add_user = sqlx::query!(r#"INSERT into users (first_name, last_name, email, password, tel, gender, key) values($1,$2, $3, $4, $5, $6, $7)"#,
                                    &f.first_name, &f.last_name, &f.email, bcrypt::hash(&f.password1, 10).unwrap(), bcrypt::hash(&f.tel, 8).unwrap(), &f.gender, &key)
                                    //.bind(hash(&f.password))
                                    .execute(&req.state().db_pool).await;
                                match add_user {
                                    Ok(_user) => {
                                        let user2: AuthUser = sqlx::query_as("SELECT * FROM users WHERE email = $1")
                                            .bind(&f.email)
                                            //.bind(&f.tel)
                                            //.bind(hash(&f.password))
                                            .fetch_one(&req.state().db_pool).await?;
                                        res.set_body(Body::from_json(&user2)?);
                                        Ok(res)
                                    }
                                    Err(_) => {

                                        let res = tide::Response::new(StatusCode::BadRequest);
                                        Ok(res)
                                    }
                                }
                            }
                            Err(_)=>{
                                if req.url().domain().is_none(){
                                    let add_user = sqlx::query!(r#"INSERT into users (first_name, last_name, email, password, tel, gender, key) values($1,$2, $3, $4, $5, $6, $7)"#,
                                    &f.first_name, &f.last_name, &f.email, bcrypt::hash(&f.password1, 10).unwrap(), bcrypt::hash(&f.tel, 8).unwrap(), &f.gender, &key)
                                        //.bind(hash(&f.password))
                                        .execute(&req.state().db_pool).await;
                                    match add_user {
                                        Ok(_user) => {
                                            let user2: AuthUser = sqlx::query_as("SELECT * FROM users WHERE email = $1")
                                                .bind(&f.email)
                                                //.bind(&f.tel)
                                                //.bind(hash(&f.password))
                                                .fetch_one(&req.state().db_pool).await?;
                                            let mut res = tide::Response::new(StatusCode::Ok);
                                            res.set_body(Body::from_json(&user2)?);
                                            Ok(res)
                                        }
                                        Err(_) => {
                                            let res = tide::Response::new(StatusCode::BadRequest);
                                            Ok(res)
                                        }
                                    }
                                }
                                else{
                                    let res = tide::Response::new(StatusCode::BadRequest);
                                    Ok(res)
                                }

                            }
                        }
                    }
                }
            }
        },
        Err(_e) => {
            let mut res = tide::Response::new(StatusCode::Unauthorized);
            res.set_body(Body::from_json(&"Girdiğiniz bilgiler eksik veya yanlış")?);
            Ok(res)
        }
    }

}

