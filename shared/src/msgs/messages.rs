use moonlight::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum MessagesUpMsgs{
    SendMessage(NewMessage),
    GetNewMessages(i32, i32),
    GetMessages(i32),
    //ReadMessage
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum MessagesDownMsgs{
    SentMessage(Message),
    SendMessageErr(String),
    GetMessages(Vec<Message>),
    GetNewMessages(Vec<Message>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Message{
    pub id: i32,
    pub sender_id: i32,
    pub school_id: Option<i32>,
    pub school_name: String,
    pub body: String,
    pub send_time: NaiveDateTime,
    pub to_school: bool,
    pub read: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct NewMessage{
    pub sender_id: i32,
    pub school_id: Option<i32>,
    pub school_name: String,
    pub body: String,
    pub send_time: NaiveDateTime,
    pub to_school: bool,
    pub read: bool
}
