use std::env;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;
use koibumi_base32::encode;

// gera codigo, aquele codigo add ele no bc e manda para o usuario
pub fn gerar_segredo() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.r#gen();
    encode(&bytes)
}


fn envia(email: Message){

    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER not set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");

    let creds = Credentials::new(smtp_user, smtp_password);
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();


    let _ = mailer.send(&email);
}

pub fn send_email_senha (email: &String) {
    let url = "http://127.0.0.1:8000/static/html/altera_senha_esqueci_email.html";
    let msg: String = format!("Ola Clique no link para alterar sua senha: {}", url);

    let email = Message::builder()
        .from("PUCBank <no-reply@labcyber.xyz>".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Verifique Seu Email")
        .body(msg)
        .unwrap();
    envia(email);

}

pub fn send_verification(email:&String ,nome:&String, codigo_autenticador_usr:&String) {
    
    let verification_url: &str = "http://127.0.0.1:8000/static/html/conf_email.html";

    let msg: String = format!("Ola {}\nPara voce ter acesso a sua conta futuramente, 
    adicione esse codigo em seu aplicativo de autenticador: {}\nClique no link para verificar seu email: {}", 
    nome, codigo_autenticador_usr, verification_url);
    
    let email = Message::builder()
        .from("PUCBank <no-reply@labcyber.xyz>".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Verifique Seu Email")
        .body(msg)
        .unwrap();
    envia(email);

}