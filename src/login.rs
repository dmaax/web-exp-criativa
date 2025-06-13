use rocket::serde::{ json::Json };
use rocket::post;
use diesel::prelude::*;
use crate::schema::usuarios::dsl::*;
use crate::login_db::conectar_escritor_leitor;
use crate::models::Usuario;
use rocket::http::{Cookie, CookieJar};
use openssl::rsa::Rsa;
use openssl::symm::{decrypt, Cipher};
#[allow(deprecated)]
use base64::{decode as base64_decode};
use serde::Deserialize as SerdeDeserialize;
use rocket::serde::json::serde_json;


use crate::chave::obter_chave_privada;


#[derive(Debug, SerdeDeserialize)]
pub struct EncryptedPayload {
    chave_aes_criptografada: String,
    iv: String,
    mensagem_criptografada: String,
}

#[derive(Debug, SerdeDeserialize)]
struct CredenciaisLoginDescriptografado {
    email: String,
    senha: String,
}


#[post("/login", format = "json", data = "<payload>")]
pub fn verificar_login(payload: Json<EncryptedPayload>, cookies: &CookieJar<'_>) -> Json<bool> {
    // Descriptografa a chave AES
    let chave_privada_pem = obter_chave_privada();
    let rsa = Rsa::private_key_from_pem(&chave_privada_pem.as_bytes()).expect("Erro ao carregar chave privada");

    #[allow(deprecated)]
    let chave_aes_criptografada = base64_decode(&payload.chave_aes_criptografada).unwrap();
    let mut chave_aes_base64 = vec![0; rsa.size() as usize];
    let chave_aes_base64_len = rsa.private_decrypt(&chave_aes_criptografada, &mut chave_aes_base64, openssl::rsa::Padding::PKCS1).unwrap();
    chave_aes_base64.truncate(chave_aes_base64_len);

    let chave_aes_base64_str = String::from_utf8(chave_aes_base64).unwrap();
    #[allow(deprecated)]
    let chave_aes = base64_decode(&chave_aes_base64_str).unwrap();

    #[allow(deprecated)]
    let iv = base64_decode(&payload.iv).unwrap();

    #[allow(deprecated)]
    let mensagem_criptografada = base64_decode(&payload.mensagem_criptografada).unwrap();

    let decrypted_data = decrypt(
        Cipher::aes_256_cbc(),
        &chave_aes,
        Some(&iv),
        &mensagem_criptografada
    ).unwrap();

    let decrypted_json = String::from_utf8(decrypted_data).unwrap();

    let credenciais: CredenciaisLoginDescriptografado = match serde_json::from_str(&decrypted_json) {
        Ok(d) => d,
        Err(_) => return Json(false),
    };

    let mut conn = conectar_escritor_leitor();
    let resultado = usuarios
        .filter(email.eq(&credenciais.email))
        .first::<Usuario>(&mut conn)
        .optional();

    match resultado {
        Ok(Some(usuario)) => {
            if credenciais.senha == usuario.senha_hash {
                cookies.add(Cookie::new("user_id", usuario.id.to_string()));
                Json(true)
            } else {
                Json(false)
            }
        },
        _ => Json(false),
    }
}