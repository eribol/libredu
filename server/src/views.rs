use tide::{Request};
use tide::StatusCode;
use serde::*;
use crate::AppState;
use sqlx::types::chrono::NaiveDateTime;
use http_types::{Body, Cookie};
use crate::request::{Auth, SchoolAuth};
use crate::model::timetable::Day;
//use crate::model::class::Class;
//use crate::model::activity::Activity;
use crate::model::school::{SchoolDetail};
use uuid::Uuid;
use lettre::{Transport, ClientSecurity};
use crate::model::post::SchoolPost;
use crate::model::city::{City, Town};


//use crate::request;
//use shared::models::user::AuthUser;
#[derive(Clone, sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub email: String,
    pub password: String,
    pub date_join: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_staff: Option<bool>,
    pub is_admin: bool,
    pub tel: Option<String>,
    pub gender:Option<String>,
    pub img:Option<String>,
}


#[derive(sqlx::FromRow,Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthUser{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: Option<String>,
    pub is_admin: bool,
    //pub is_active: bool,
    //pub is_staff: bool,
}
pub async fn logout(_req: Request<AppState>) -> tide::Result{
    let mut res = tide::Response::new(StatusCode::Ok);
    res.remove_cookie(Cookie::named("libredu-user"));
    res.remove_cookie(Cookie::named("libredu-uuid"));
    res.set_body(Body::from_file("./server/templates/index.html").await?);
    res.insert_header("content-type", "text/html");
    Ok(res)
}
pub async fn reset_password(_req: Request<AppState>) -> tide::Result{
    let mut res = tide::Response::new(StatusCode::Ok);
    res.set_body(Body::from_file("./server/templates/index.html").await?);
    res.insert_header("content-type", "text/html");
    Ok(res)
}
pub async fn robots(_req: Request<AppState>) -> tide::Result{
    let mut res = tide::Response::new(StatusCode::Ok);
    res.set_body(Body::from_file("./server/templates/robots.txt").await?);
    Ok(res)
}
pub async fn favico(_req: Request<AppState>) -> tide::Result{
    let mut res = tide::Response::new(StatusCode::Ok);
    res.set_body(Body::from_file("./server/templates/favicon.ico").await?);
    Ok(res)
}

#[derive(Deserialize, Serialize)]
pub struct ResetPassword{
    pub email: String,
    pub tel: String,
    pub key: String,
    pub password1: String,
    pub password2: String
}
pub async fn post_reset(mut req: Request<AppState>) -> tide::Result{
    match req.user().await{
        Ok(_user)=>{
            let res = tide::Response::new(StatusCode::Unauthorized);
            Ok(res)
        }
        Err(_)=>{
            use sqlx_core::postgres::PgQueryAs;
            let reset_form: ResetPassword = req.body_json().await?;
            let user: sqlx::Result<AuthUser> = sqlx::query_as("SELECT * FROM users where email = $1 and (tel = $2 or tel = $3) and key = $4")
                .bind(&reset_form.email)
                .bind(&reset_form.tel)
                .bind(&bcrypt::hash(reset_form.tel, 8)?)
                .bind(&reset_form.key)
                .fetch_one(&req.state().db_pool).await;
            match user{
                Ok(_u)=>{
                    if reset_form.password1 == reset_form.password2{
                        let mut res = tide::Response::new(StatusCode::Ok);
                        let _user: sqlx::Result<AuthUser> = sqlx::query_as("update users set password = $1 where email = $2")
                            .bind(bcrypt::hash(&reset_form.password1, 10)?)
                            .bind(&reset_form.email)
                            .fetch_one(&req.state().db_pool).await;
                        res.set_body(Body::from_json(&"Şifreniz güncellendi")?);
                        Ok(res)
                    }
                    else{
                        let res = tide::Response::new(StatusCode::NotAcceptable);
                        Ok(res)
                    }
                }
                Err(_)=>{
                    let res = tide::Response::new(StatusCode::NotAcceptable);
                    Ok(res)
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
struct Key{
    key:Option<String>
}
pub async fn send_key(mut req: Request<AppState>) -> tide::Result{
    let email: Result<String,_> = req.body_json().await;
    match email{
        Ok(e) => {
            use lettre::{SendableEmail, Envelope, EmailAddress, SmtpClient};
            use sqlx_core::postgres::PgQueryAs;
            let user: sqlx::Result<Key> = sqlx::query_as("SELECT key FROM users WHERE email = $1")
                .bind(&e)
                .fetch_one(&req.state().db_pool).await;
            match user{
                Ok(u)=>{

                    match u.key{
                        Some(k)=>{
                            let send_email = SendableEmail::new(
                                Envelope::new(
                                    Some(EmailAddress::new("admin@libredu.org".to_string()).unwrap()),
                                    vec![EmailAddress::new(e.clone()).unwrap()],
                                ).unwrap(),
                                "id".to_string(),
                                k.clone().into_bytes(),
                            );
                            let mut mailer =
                                SmtpClient::new(("127.0.0.1", 25), ClientSecurity::None).unwrap().transport();
                            // Send the email
                            let result = mailer.send(send_email);
                            match result{
                                Ok(_)=>{
                                    let res = tide::Response::new(StatusCode::Ok);
                                    let _user: Key = sqlx::query_as("UPDATE users set key = $1 WHERE email = $2 returning key")
                                        .bind(&k)
                                        .bind(e)
                                        .fetch_one(&req.state().db_pool).await?;
                                    Ok(res)
                                }
                                Err(_)=> {
                                    let res = tide::Response::new(StatusCode::InternalServerError);
                                    Ok(res)
                                }
                            }
                        }
                        None=>{
                            let key = Uuid::new_v4().to_string();
                            let send_email = SendableEmail::new(
                                Envelope::new(
                                    Some(EmailAddress::new("admin@libredu.org".to_string()).unwrap()),
                                    vec![EmailAddress::new(e.clone()).unwrap()],
                                ).unwrap(),
                                "id".to_string(),
                                key.clone().into_bytes(),
                            );
                            let mut mailer =
                                SmtpClient::new(("127.0.0.1", 25), ClientSecurity::None).unwrap().transport();
                            // Send the email
                            let result = mailer.send(send_email);
                            match result{
                                Ok(_)=>{
                                    let res = tide::Response::new(StatusCode::Ok);
                                    let _user: Key = sqlx::query_as("UPDATE users set key = $1 WHERE email = $2 returning key")
                                        .bind(&key)
                                        .bind(&e)
                                        .fetch_one(&req.state().db_pool).await?;
                                    Ok(res)
                                }
                                Err(_)=> {
                                    let res = tide::Response::new(StatusCode::InternalServerError);
                                    Ok(res)
                                }
                            }
                        }
                    }

                }
                Err(_)=>{
                    let res = tide::Response::new(StatusCode::NotAcceptable);
                    Ok(res)
                }
            }
        }
        Err(_) => {
            let res = tide::Response::new(StatusCode::Unauthorized);
            Ok(res)
        }
    }

}

pub async fn city(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    let cities: Vec<City> = sqlx::query_as("SELECT * FROM city")
        .fetch_all(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&cities)?);
    Ok(res)
}

pub async fn town(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    let city: i32 = req.param("city")?.parse()?;
    let towns: Vec<Town> = sqlx::query_as("SELECT * FROM town WHERE city = $1")
        .bind(&city)
        .fetch_all(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&towns)?);
    Ok(res)
}

pub async fn days(req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    let days: Vec<Day> = sqlx::query_as("SELECT * FROM days")
        .fetch_all(&req.state().db_pool).await?;
    res.set_body(Body::from_json(&days)?);
    Ok(res)
}
pub async fn get_posts(req: Request<AppState>) -> tide::Result {
    use crate::model::post::Post;
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    let posts: Vec<Post> = sqlx::query_as("SELECT * from post order by pub_date desc limit 40")
        .fetch_all(&req.state().db_pool).await?;
    let mut school_posts: Vec<SchoolPost> = vec![];
    for p in posts{
        match p.school{
            Some(id) =>{
                let sch = SchoolDetail::get(&req, id).await?;
                let school_post = SchoolPost{
                    id: p.id,
                    body: p.body,
                    pub_date: p.pub_date,
                    school: Some(sch),
                    sender: p.sender
                };
                school_posts.push(school_post);
            }
            None => {
                let school_post = SchoolPost{
                    id: p.id,
                    body: p.body,
                    pub_date: p.pub_date,
                    school: None,
                    sender: p.sender
                };
                school_posts.push(school_post);
            }
        }
    }
    res.set_body(Body::from_json(&school_posts)?);
    Ok(res)
}

pub async fn posts(mut req: Request<AppState>) -> tide::Result {
    use crate::model::post::{Post, NewPost};
    use sqlx_core::postgres::PgQueryAs;
    let mut form: NewPost = req.body_json().await?;
    let school_auth: &SchoolAuth = req.ext().unwrap();
    if school_auth.role < 3 {
        let mut res = tide::Response::new(StatusCode::Ok);
        form.body = form.body.replace("\n", "<br>");
        let user = req.user().await.unwrap();
        if user.is_admin {
            let post: Post = sqlx::query_as("insert into post(body, sender, pub_date) values($1, $2, $3) returning id, body, pub_date, school, sender")
                .bind(&form.body)
                .bind(&user.id)
                .bind(&chrono::Utc::now())
                .fetch_one(&req.state().db_pool).await?;
            let send_post = SchoolPost {
                id: post.id,
                body: post.body,
                pub_date: post.pub_date,
                school: None,
                sender: post.sender
            };
            res.set_body(Body::from_json(&send_post)?);
            Ok(res)
        } else {
            let post: Post = sqlx::query_as("insert into post(body, sender, pub_date, school) values($1, $2, $3, $4) returning id, body, pub_date, school, sender")
                .bind(&form.body)
                .bind(&user.id)
                .bind(&chrono::Utc::now())
                .bind(&school_auth.school.id)
                .fetch_one(&req.state().db_pool).await?;
            let send_post = SchoolPost {
                id: post.id,
                body: post.body,
                pub_date: post.pub_date,
                school: Some(school_auth.school.clone()),
                sender: post.sender
            };
            res.set_body(Body::from_json(&send_post)?);
            Ok(res)
        }
    }
    else {
        let res = tide::Response::new(StatusCode::Unauthorized);
        Ok(res)
    }
}

pub async fn del_post(req: Request<AppState>) -> tide::Result {
    use crate::model::post::{Post};
    let mut res = tide::Response::new(StatusCode::Ok);
    let post_id: i32 = req.param("post_id")?.parse()?;
    use sqlx_core::postgres::PgQueryAs;
    let user = req.user().await?;
    let get_post: Post = sqlx::query_as("SELECT * FROM post where id = $1")
        .bind(&post_id)
        .fetch_one(&req.state().db_pool).await?;
    if user.is_admin || get_post.sender == user.id {
        sqlx::query("delete FROM post where id = $1")
            .bind(&post_id)
            .execute(&req.state().db_pool).await?;
        res.set_body(Body::from_json(&post_id)?);
        Ok(res)
    } else {
        Ok(res)
    }
}



/*pub async fn add_cities(mut req: Request<AppState>) -> tide::Result {
    let mut res = tide::Response::new(StatusCode::Ok);
    use sqlx_core::postgres::PgQueryAs;
    use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};
    let mut excel: Xlsx<_> = open_workbook("./server/iller.xlsx").unwrap();
    let mut last_city = 0;
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            if row[0].get_float().unwrap() as i32 == last_city{
                let town = Town{
                    city: last_city,
                    pk: row[2].get_float().unwrap() as i32,
                    name: row[3].to_string()
                };

                let town: sqlx::Result<Town> = sqlx::query_as("insert into town(pk, city, name) values($1, $2, $3) returning pk, city, name")
                    .bind(&town.pk)
                    .bind(&town.city)
                    .bind(&town.name)
                    .fetch_one(&req.state().db_pool).await;
            }
            else{
                let city = City{ pk: row[0].get_float().unwrap() as i32, name: row[1].to_string() };
                let city: sqlx::Result<City> = sqlx::query_as("insert into city(pk, name) values($1, $2) returning pk, name")
                    .bind(&city.pk)
                    .bind(&city.name)
                    .fetch_one(&req.state().db_pool).await;
                last_city = row[0].get_float().unwrap() as i32;
                let town = Town{
                    city: last_city,
                    pk: row[2].get_float().unwrap() as i32,
                    name: row[3].to_string()
                };
                let town: sqlx::Result<Town> = sqlx::query_as("insert into town(pk, city, name) values($1, $2, $3) returning pk, city, name")
                    .bind(&town.pk)
                    .bind(&town.city)
                    .bind(&town.name)
                    .fetch_one(&req.state().db_pool).await;
              //  println!("city = {:?},town = {:?}", city, town);
            }

        }
        Ok(res)
    }
    else{
        Ok(res)
    }

}*/