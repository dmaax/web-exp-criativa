use rocket::{post, http::Status, serde::json::Json};
use serde::Deserialize;
use diesel::prelude::*;
use openssl::rsa::Rsa;
use openssl::symm::{decrypt, Cipher};
#[allow(deprecated)]
use base64::{decode as base64_decode};
use crate::chave::obter_chave_privada;
use crate::schema::usuarios;
use crate::login_db::conectar_escritor_leitor;
use crate::mail;
use crate::models::Usuario;
use rocket::serde::json::serde_json;

#[derive(Deserialize)]
pub struct EncryptedPayload {
    pub chave_aes_criptografada: String,
    pub iv: String,
    pub mensagem_criptografada: String,
}

#[derive(Deserialize)]
pub struct EsqueciSenhaRequest {
    pub email: String,
}

#[post("/esqueci_senha", format = "json", data = "<payload>")]
pub async fn esqueci_senha(payload: Json<EncryptedPayload>) -> Result<Status, Status> {
    // Descriptografa a chave AES
    let chave_privada_pem = obter_chave_privada();
    let rsa = Rsa::private_key_from_pem(&chave_privada_pem.as_bytes())
        .map_err(|_| Status::InternalServerError)?;

    #[allow(deprecated)]
    let chave_aes_criptografada = base64_decode(&payload.chave_aes_criptografada)
        .map_err(|_| Status::BadRequest)?;
    let mut chave_aes_base64 = vec![0; rsa.size() as usize];
    let chave_aes_base64_len = rsa.private_decrypt(
        &chave_aes_criptografada,
        &mut chave_aes_base64,
        openssl::rsa::Padding::PKCS1
    ).map_err(|_| Status::InternalServerError)?;

    let chave_aes_base64_str = String::from_utf8(chave_aes_base64[..chave_aes_base64_len].to_vec())
        .map_err(|_| Status::BadRequest)?;
    #[allow(deprecated)]
    let chave_aes = base64_decode(&chave_aes_base64_str)
        .map_err(|_| Status::BadRequest)?;

    #[allow(deprecated)]
    let iv = base64_decode(&payload.iv)
        .map_err(|_| Status::BadRequest)?;
    #[allow(deprecated)]
    let mensagem_criptografada = base64_decode(&payload.mensagem_criptografada)
        .map_err(|_| Status::BadRequest)?;

    let dados_descriptografados = decrypt(
        Cipher::aes_256_cbc(),
        &chave_aes,
        Some(&iv),
        &mensagem_criptografada
    ).map_err(|_| Status::InternalServerError)?;

    let json_descriptografado = String::from_utf8(dados_descriptografados)
        .map_err(|_| Status::BadRequest)?;

    let req: EsqueciSenhaRequest = serde_json::from_str(&json_descriptografado)
        .map_err(|_| Status::BadRequest)?;

    let mut conn = conectar_escritor_leitor();
    let result = usuarios::dsl::usuarios
        .filter(usuarios::dsl::email.eq(&req.email))
        .first::<Usuario>(&mut conn)
        .optional();

    match result {
        Ok(Some(_user)) => { 

            let _ = mail::send_email_senha(&req.email);

            Ok(Status::Ok)
        }
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
