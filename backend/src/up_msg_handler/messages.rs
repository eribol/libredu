use moon::tokio_stream::StreamExt;
use sqlx::Row;
use moon::AuthToken;
use shared::msgs::admin::{AdminUpMsgs, AdminDownMsgs};
use shared::DownMsg;
use shared::msgs::messages::{MessagesUpMsgs, MessagesDownMsgs, Message};
use sqlx::types::chrono;

use crate::connection::admin::get_schools;
use crate::connection::get_user;
use crate::connection::school::get_school;

use super::auth::POSTGRES;

pub async fn message(msg: MessagesUpMsgs, auth_token: Option<AuthToken>)->DownMsg{
    let mut d_msg = DownMsg::AuthError("Not auth".to_string());
    if let Some(auth) = auth_token{
        let user = get_user(auth.as_str()).await.unwrap();
        match msg{
            MessagesUpMsgs::SendMessage(form) => {
                let m_msg = new_message(form, user).await; 
                return DownMsg::Messages(m_msg);
            }
            MessagesUpMsgs::GetMessages => return DownMsg::Messages(MessagesDownMsgs::GetMessages(get_messages(user).await))
        }
    }
   d_msg
}

pub async fn new_message(form: Message, user: i32)-> MessagesDownMsgs{
    let db = POSTGRES.read().await;
    let school = get_school(user).await.unwrap();
    let mut a = sqlx::query(r#"insert into help_messages(sender_id, school_name,school_id, body, sent, send_time, receiver_id) 
        values($1, $2, $3, $4, $5, $6, $7) returning sender_id, school_name,school_id, body, sent, send_time, receiver_id"#,
    )
    .bind(&user)
    .bind(&school.name)
    .bind(&school.id)
    .bind(&form.body)
    .bind(&form.sent)
    .bind(&form.send_time)
    .bind(&form.receiver_id)
    .fetch(&*db);
    if let Some(row) = a.try_next().await.unwrap(){
        let m = Message{
            sender_id: row.try_get("sender_id").unwrap(),
            receiver_id: row.try_get("receiver_id").unwrap_or(user),
            school_id: row.try_get("school_id").unwrap(),
            school_name: row.try_get("school_name").unwrap(),
            body: row.try_get("body").unwrap(),
            send_time: row.try_get("send_time").unwrap(),
            sent: row.try_get("sent").unwrap()
        };
        return MessagesDownMsgs::SentMessage(m);
    }
    return MessagesDownMsgs::SendMessageErr("Database error".to_string())
}

pub async fn get_messages(user: i32)-> Vec<Message>{
    let db = POSTGRES.read().await;
    let school = get_school(user).await.unwrap();
    let mut query = sqlx::query(r#"select * from help_messages where school_id = $1"#,
    )
    .bind(&school.id)
    .fetch(&*db);
    let mut msgs = vec![];
    while let Some(row) = query.try_next().await.unwrap(){
        let m = Message{
            sender_id: row.try_get("sender_id").unwrap(),
            receiver_id: row.try_get("receiver_id").unwrap_or(user),
            school_id: row.try_get("school_id").unwrap(),
            school_name: row.try_get("school_name").unwrap(),
            body: row.try_get("body").unwrap(),
            send_time: row.try_get("send_time").unwrap(),
            sent: row.try_get("sent").unwrap()
        };
        msgs.push(m);
    }
    msgs
}