use std::env;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;
use koibumi_base32::encode;

// gera codigo
#[allow(dead_code)]
pub fn gerar_segredo() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.r#gen();
    encode(&bytes)
}

//use rand::{distributions::Alphanumeric, Rng};

//use crate::newcodmfa; dx isso aq 


#[derive(serde::Deserialize)]
pub struct EmailRequest {
    email: Box<str>,
    nome : Box<str>,
}

/* 
fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32) // Generate a 32-character random token
        .map(char::from)
        .collect()
}
*/

pub fn send_verification(email:String ,nome:String) {
    
    //let token = generate_token();
    //let verification_url = format!("https://bank.labcyber.xyz/verify?token={}", token);
    //vai ficar assim ate ficar pronto
    let verification_url: &str = "http://127.0.0.1:8000/static/html/conf_email.html";

    //let codigo_autenticador_usr: String = newcodmfa::gerar_segredo();
    // futuramente add essa linha, agora vai ficar uma "senha" fixa para mostrar na primiera sprint
    let codigo_autenticador_usr: &str = "ea273b66in5pvp64sg2gigpwuu";

    let msg: String = format!("Ola {}\nPara voce ter acesso a sua conta futuramente, 
    adicione esse codigo em seu aplicativo de autenticador: {}\nClique no link para verificar seu email: {}", 
    nome, codigo_autenticador_usr, verification_url);
    
    let email = Message::builder()
        .from("PUCBank <no-reply@labcyber.xyz>".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Verifique Seu Email")
        .body(msg)
        .unwrap();

    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER not set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");

    let creds = Credentials::new(smtp_user, smtp_password);
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();


    mailer.send(&email);

}