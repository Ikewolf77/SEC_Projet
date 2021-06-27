use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

const SMTP_USER: &str = "USER";
const SMTP_PASS: &str = "PASS";
const SMTP_SERV: &str = "SERV";

pub fn send_password_mail(dst: &str, code: &str) {
    let email = Message::builder()
        .from("KING <nobody@localhost>".parse().unwrap())
        .reply_to("KING <nobody@localhost>".parse().unwrap())
        .to(dst.parse().unwrap())
        .subject("Reset your password for KING")
        .body(format!("Your code : {}", code))
        .unwrap();

    let creds = Credentials::new(SMTP_USER.to_string(), SMTP_PASS.to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(SMTP_SERV)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => return,
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
