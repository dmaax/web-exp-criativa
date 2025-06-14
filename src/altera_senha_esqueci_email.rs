use rocket::serde::{Deserialize, json::Json};
use rocket::post;
use rocket::http::Status;
use diesel::prelude::*;
use crate::login_db::conectar_escritor_leitor;
use crate::schema::usuarios::dsl::*;
use crate::models::Usuario;
use koibumi_base32 as base32;
use totp_lite::{totp_custom, Sha1, DEFAULT_STEP};
use std::time::SystemTime;
use openssl::rsa::Rsa;
use openssl::symm::{decrypt, Cipher};
#[allow(deprecated)]
use base64::{decode as base64_decode};
use crate::chave::obter_chave_privada;
use rocket::serde::json::serde_json;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AlterarSenhaEmail {
    pub cpf: String,
    pub mfa: String,
    pub nova_senha: String,
}

#[derive(Debug, Deserialize)]
pub struct EncryptedPayload {
    pub chave_aes_criptografada: String,
    pub iv: String,
    pub mensagem_criptografada: String,
}


fn gerar_codigo_totp(base32_secret: &str) -> Option<String> {
    let seconds: u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()?
        .as_secs();
    let secret = base32::decode(&base32_secret.trim().to_lowercase()).ok()?;
    Some(totp_custom::<Sha1>(DEFAULT_STEP, 6, &secret, seconds))
}

#[post("/alterar_senha_email", format = "json", data = "<payload>")]
pub async fn alterar_senha_email(payload: Json<EncryptedPayload>) -> Result<Json<bool>, Status> {
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
    let iv = base64_decode(&payload.iv).map_err(|_| Status::BadRequest)?;
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

    let dados: AlterarSenhaEmail = serde_json::from_str(&json_descriptografado)
        .map_err(|_| Status::BadRequest)?;

    let mut conn = conectar_escritor_leitor();

    let cpf_limpo = dados.cpf.replace(|c: char| !c.is_numeric(), "");

    let usuario_result = usuarios
        .filter(cpf.eq(&cpf_limpo))
        .first::<Usuario>(&mut conn)
        .optional();

    if let Ok(Some(usuario)) = usuario_result {
        if let Some(codigo_gerado) = gerar_codigo_totp(&usuario.codigo_2fa) {
            if dados.mfa.trim() == codigo_gerado {
                let resultado_atualizacao = diesel::update(usuarios.filter(cpf.eq(&cpf_limpo)))
                    .set(senha_hash.eq(&dados.nova_senha))
                    .execute(&mut conn);

                if resultado_atualizacao.is_ok() {
                    return Ok(Json(true));
                } else {
                    return Err(Status::InternalServerError);
                }
            }
        }
    }

    Ok(Json(false))
}
