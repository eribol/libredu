use lettre::message::header::ContentType;
use lettre::{
    AsyncSendmailTransport, AsyncTransport, Message, SendmailTransport, Tokio1Executor,
};
use shared::DownMsg;

pub async fn send_mail(email: String, body: String)->DownMsg{
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
    match sender.send(email).await{
        Ok(_)=> return DownMsg::Signin,
        Err(_)=>return DownMsg::SigninError("Signin Error".to_string())
    }
}