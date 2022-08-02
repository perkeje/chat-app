use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use std::env;

use crate::models::user::User;

pub async fn send_confirmation_email(user: &User) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_credentials = Credentials::new(
        env::var("EMAIL_USER").unwrap(),
        env::var("EMAIL_PASS").unwrap(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(smtp_credentials)
        .build();

    let token = crate::services::jwt::generate(user);
    let url = format!("http://localhost:8080/confirm/{}", token);

    let from = env::var("EMAIL_USER").unwrap();
    let to = &user.email;
    let subject = "Confirmation email";
    let body = format!("Please conffirm email at {}", url);

    send_email_smtp(&mailer, &from, &to, subject, body).await
}

async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    mailer.send(email).await?;

    Ok(())
}
