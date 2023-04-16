use lettre::message::header::ContentType;
use lettre::{Message, SendmailTransport, Transport};

pub fn send_mail(sender: String, email: String, password: String, body: String){
let email = Message::builder()
    .from(
        format!(r#"{sender:?}@gmail.com"#).parse().unwrap()
    )
    .to(email.parse().unwrap())
    .subject("Üyelik Onay")
    .header(ContentType::TEXT_HTML)
    .body(body)
    .unwrap();
    let sender = SendmailTransport::new();
    let result = sender.send(&email);
}