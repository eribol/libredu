use lettre::message::header::ContentType;
use lettre::{
    AsyncSendmailTransport, AsyncTransport, Message, Tokio1Executor,
};
use shared::DownMsg;

pub async fn send_mail(email: String, body: String)->DownMsg{
let email = Message::builder()
    .from(
        r#"info@libredu.org"#.to_string().parse().unwrap()
    )
    .to(email.parse().unwrap())
    .subject("Üyelik Onay")
    .header(ContentType::TEXT_HTML)
    .body(body)
    .unwrap();
    
    let sender = AsyncSendmailTransport::<Tokio1Executor>::new();
    match sender.send(email).await{
        Ok(_)=> DownMsg::Signin,
        Err(_)=>DownMsg::SigninError("Signin Error".to_string())
    }
}