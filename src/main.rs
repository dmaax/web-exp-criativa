#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, NamedFile};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::http::Status;
use std::env;
use std::path::Path;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::{distributions::Alphanumeric, Rng};
use dotenv::dotenv;

#[derive(serde::Deserialize)]
struct EmailRequest {
    email: String,
}

fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32) // Generate a 32-character random token
        .map(char::from)
        .collect()
}

#[get("/")]
fn root() -> Redirect {
    Redirect::to(uri!("/index"))
}

#[get("/<file>")]
async fn html_files(file: &str) -> Option<NamedFile> {
    let path = format!("static/{}.html", file);
    NamedFile::open(Path::new(&path)).await.ok()
}

#[post("/send_verification", format = "json", data = "<request>")]
async fn send_verification(request: Json<EmailRequest>) -> Result<&'static str, Status> {
    let email_address = request.email.trim();

    if !email_address.contains('@') {
        return Err(Status::BadRequest);
    }

    let token = generate_token();
    let verification_url = format!("https://bank.labcyber.xyz/verify?token={}", token);

    let email = Message::builder()
        .from("CyberBank <no-reply@labcyber.xyz.com>".parse().unwrap())
        .to(email_address.parse().unwrap())
        .subject("Verifique Seu Email")
        .body(format!("Clique no link para verificar seu email: {}", verification_url))
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

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let _smtp_user = env::var("SMTP_USER").expect("SMTP_USER não configurado");
    let _smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD não configurado");

    rocket::build()
        .mount("/", routes![root, html_files, send_verification])
        .mount("/static", FileServer::from("static"))
}
