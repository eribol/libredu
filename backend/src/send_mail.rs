use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

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
    let tls = lettre::transport::smtp::client::TlsParameters::builder("smtp-relay.gmail.com".to_owned())
    .dangerous_accept_invalid_certs(true)
    .build().unwrap();
    let creds = Credentials::new(sender.to_owned(), password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp-relay.gmail.com")
        .unwrap()
        .credentials(creds)
        .tls(lettre::transport::smtp::client::Tls::Required(tls))
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}