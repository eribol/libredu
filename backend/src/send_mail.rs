use dotenvy;
use lettre::message::header::ContentType;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use shared::DownMsg;

pub async fn send_mail(email: String, body: String, _subject: String) -> DownMsg {
    dotenvy::dotenv().unwrap();
    let gmail_user = std::env::var("GMAIL_USERNAME").unwrap();
    let gmail_password = std::env::var("GMAIL_PASSWORD").unwrap();
    let email = Message::builder()
        .from(r#"info@libredu.org"#.to_string().parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Libredu Hesap Etkinleştirme")
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
    match mailer.send(email).await {
        Ok(_) => DownMsg::Signin,
        Err(e) => DownMsg::SigninError(e.to_string()),
    }
}
