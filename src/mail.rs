use rocket::serde::json::Json;
use rocket::http::Status;
use std::env;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::{distributions::Alphanumeric, Rng};


#[derive(serde::Deserialize)]
pub struct EmailRequest {
    email: String,
    nome : String,
}


fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32) // Generate a 32-character random token
        .map(char::from)
        .collect()
}

#[post("/send_verification", format = "json", data = "<request>")]
pub async fn send_verification(request: Json<EmailRequest>) -> Result<&'static str, Status> {
    let email_address = request.email.trim();
    let nome_pessoa: &str = request.nome.trim();
    if !email_address.contains('@') {
        return Err(Status::BadRequest);
    }

    let token = generate_token();
    let verification_url = format!("https://bank.labcyber.xyz/verify?token={}", token);

    let email = Message::builder()
        .from("PUCBank <no-reply@labcyber.xyz>".parse().unwrap())
        .to(email_address.parse().unwrap())
        .subject("Verifique Seu Email")
        .body(format!("Ola {}, Clique no link para verificar seu email: {}",nome_pessoa, verification_url))
        .unwrap();

    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER not set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");

    let creds = Credentials::new(smtp_user, smtp_password);
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok("Email Enviado!"),
        Err(_) => Err(Status::InternalServerError),
    }
}