use moonlight::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum MessagesUpMsgs{
    SendMessage(Message),
    GetMessages
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub enum MessagesDownMsgs{
    SentMessage(Message),
    SendMessageErr(String),
    GetMessages(Vec<Message>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Message{
    pub sender_id: i32,
    pub receiver_id: i32,
    pub school_id: Option<i32>,
    pub school_name: String,
    pub body: String,
    pub send_time: NaiveDateTime,
    pub sent: i16
}
