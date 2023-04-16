use lettre::message::header::ContentType;
use lettre::{Message, SendmailTransport, Transport};
use lettre::{
    AsyncSendmailTransport, AsyncTransport, Message, SendmailTransport, Tokio1Executor,
};

pub fn send_mail(email: String, body: String){
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
    let result = sender.send(email).await;
}