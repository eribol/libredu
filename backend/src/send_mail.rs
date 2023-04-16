use lettre::message::header::ContentType;
use lettre::{
    AsyncSendmailTransport, AsyncTransport, Message, SendmailTransport, Tokio1Executor,
};

pub async fn send_mail(email: String, body: String){
let email = Message::builder()
    .from(
        format!(r#"info@libredu.org"#).parse().unwrap()
    )
    .to(email.parse().unwrap())
    .subject("Üyelik Onay")
    .header(ContentType::TEXT_HTML)
    .body(body)
    .unwrap();
    
    let sender = AsyncSendmailTransport::<Tokio1Executor>::new();
    sender.send(email).await.unwrap();
}