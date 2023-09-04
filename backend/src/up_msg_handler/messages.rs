use moon::tokio_stream::StreamExt;
use sqlx::Row;
use moon::AuthToken;
use shared::DownMsg;
use shared::msgs::messages::{MessagesUpMsgs, MessagesDownMsgs, Message, NewMessage};

use crate::connection::get_user;
use crate::connection::school::get_school;

use super::auth::POSTGRES;

pub async fn message(msg: MessagesUpMsgs, auth_token: Option<AuthToken>)->DownMsg{
    let d_msg = DownMsg::AuthError("Not auth".to_string());
    if let Some(auth) = auth_token{
        let user = get_user(auth.as_str()).await.unwrap();
        match msg{
            MessagesUpMsgs::SendMessage(form) => {
                let m_msg = new_message(form, user).await; 
                return DownMsg::Messages(m_msg);
            }
            MessagesUpMsgs::GetMessages(school_id) => return DownMsg::Messages(MessagesDownMsgs::GetMessages(get_messages(school_id).await)),
            MessagesUpMsgs::GetNewMessages(school_id, id) => return DownMsg::Messages(MessagesDownMsgs::GetNewMessages(get_new_messages(school_id, id).await))
        }
    }
   d_msg
}

pub async fn new_message(form: NewMessage, user: i32)-> MessagesDownMsgs{
    let db = POSTGRES.read().await;
    let school = get_school(user).await.unwrap();
    let mut a = sqlx::query(r#"insert into help_messages(sender_id, school_name,school_id, body, send_time, to_school, read) 
        values($1, $2, $3, $4, $5, $6, $7) returning id, sender_id, school_name,school_id, body, send_time, to_school, read"#,
    )
    .bind(&user)
    .bind(&school.name)
    .bind(&school.id)
    .bind(&form.body)
    .bind(&form.send_time)
    .bind(&form.to_school)
    .bind(&form.read)
    .fetch(&*db);
    if let Some(row) = a.try_next().await.unwrap(){
        println!("message ekleniyor");
        let m = Message{
            id: row.try_get("id").unwrap(),
            sender_id: row.try_get("sender_id").unwrap(),
            school_id: row.try_get("school_id").unwrap(),
            school_name: row.try_get("school_name").unwrap(),
            body: row.try_get("body").unwrap(),
            send_time: row.try_get("send_time").unwrap(),
            to_school: row.try_get("to_school").unwrap(),
            read: row.try_get("read").unwrap()
        };
        return MessagesDownMsgs::SentMessage(m);
    }
    return MessagesDownMsgs::SendMessageErr("Database error".to_string())
}

pub async fn get_messages(school_id: i32)-> Vec<Message>{
    let db = POSTGRES.read().await;
    let mut query = sqlx::query(r#"select * from help_messages where school_id = $1"#,
    )
    .bind(&school_id)
    .fetch(&*db);
    let mut msgs = vec![];
    while let Some(row) = query.try_next().await.unwrap(){
        //if let Some(row) = row{
            let m = Message{
                id: row.try_get("id").unwrap(),
                sender_id: row.try_get("sender_id").unwrap(),
                school_id: row.try_get("school_id").unwrap(),
                school_name: row.try_get("school_name").unwrap(),
                body: row.try_get("body").unwrap(),
                send_time: row.try_get("send_time").unwrap(),
                to_school: row.try_get("to_school").unwrap(),
                read: row.try_get("read").unwrap(),
            };
            msgs.push(m);    
        //}
    }
    msgs
}

pub async fn get_new_messages(school_id: i32, id: i32)-> Vec<Message>{
    let db = POSTGRES.read().await;
    let mut query = sqlx::query(r#"select * from help_messages where school_id = $1 and to_school = true and id > $2"#,
    )
    .bind(&school_id)
    .bind(id)
    .fetch(&*db);
    let mut msgs = vec![];
    while let Some(row) = query.try_next().await.unwrap(){
        //if let Some(row) = query{
            let m = Message{
                id: row.try_get("id").unwrap(),
                sender_id: row.try_get("sender_id").unwrap(),
                school_id: row.try_get("school_id").unwrap(),
                school_name: row.try_get("school_name").unwrap(),
                body: row.try_get("body").unwrap(),
                send_time: row.try_get("send_time").unwrap(),
                to_school: row.try_get("to_school").unwrap(),
                read: row.try_get("read").unwrap(),
            };
            msgs.push(m);
        //}
    }
    msgs
}