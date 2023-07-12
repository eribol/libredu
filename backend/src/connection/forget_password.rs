use moon::{AuthToken, EntityId};
use shared::DownMsg;

use crate::send_mail::{self, send_mail};

use super::REDISDB;

pub async fn forget_password(email: String)-> DownMsg{
    println!("fff");
    let auth_token = AuthToken::new(EntityId::new());
    let d = dotenvy::var("DOMAIN_NAME").unwrap();
    let html = create_html(d, email.clone(),  auth_token.clone().into_string());
    let _ = add_token(email.clone(), auth_token).await;
    let _register = send_mail(email.clone(), html).await;
    DownMsg::ResetPassword
}
async fn add_token(email: String, auth_token: AuthToken){
    let client = REDISDB.write().await;
    let mut con = client.get_connection().unwrap();
    let _user: String = redis::cmd("set")
        .arg(auth_token.clone().into_string())
        .arg(&email)
        .arg("EX")
        .arg(120)
        .query(&mut con).unwrap();
}

fn create_html(d: String, email: String, token: String)->String{
    let addr = format!(r"{d}/reset/{token}/{email}");
    let r = format!(r"<!DOCTYPE html>
    <html>
    <body><p>Şifrenizi yenilemek için linke <a href={addr}>tıklayın</a></p>
    
    </body>
    </html>
    ");
    println!("{r:?}");
    r
}