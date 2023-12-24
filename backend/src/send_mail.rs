use lettre::message::header::ContentType;
use lettre::{
    AsyncSendmailTransport, AsyncTransport, Message, Tokio1Executor, AsyncSmtpTransport, transport::smtp::authentication::Credentials
};
use shared::DownMsg;
use dotenvy;

pub async fn send_mail(email: String, body: String, subject: String)->DownMsg{
    dotenvy::dotenv().unwrap();
    let gmail_user = std::env::var("GMAIL_USERNAME").unwrap();
    let gmail_password = std::env::var("GMAIL_PASSWORD").unwrap();
    let email = Message::builder()
        .from(
            r#"info@libredu.org"#.to_string().parse().unwrap()
        )
        .to(email.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body)
        .unwrap();
    let creds = Credentials::new(gmail_user.to_owned(), gmail_password.to_owned());

    // Open a remote connection to gmail
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();
    //let sender = AsyncSendmailTransport::<Tokio1Executor>::new();
    match mailer.send(email).await{
        Ok(_)=> DownMsg::Signin,
        Err(_)=>DownMsg::SigninError("Signin Error".to_string())
    }
}