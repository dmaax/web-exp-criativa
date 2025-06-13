use koibumi_base32 as base32;
use std::time::SystemTime;
use totp_lite::{totp_custom, Sha1, DEFAULT_STEP};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{Deserialize, json::Json};
use rocket::request::{FromRequest, Outcome, Request};
use crate::login_db::conectar_escritor_leitor;
use crate::schema::usuarios::dsl::usuarios;
use crate::models::Usuario;
use crate::sessao;
use diesel::prelude::*;
use rocket::http::Status;
use cookie::time::{Duration, OffsetDateTime};
use rocket::http::SameSite;
use openssl::rsa::Rsa;
use openssl::symm::{decrypt, Cipher};
use rocket::serde::json::serde_json;
#[allow(deprecated)]
use base64::{decode as base64_decode};


#[derive(Debug, Deserialize)]
pub struct CodigoMfa {
    pub codigo: String,
}

pub struct ClientInfo {
    pub ip: String,
    pub user_agent: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientInfo {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let ip = req.client_ip().map(|ip| ip.to_string()).unwrap_or_default();
        let user_agent = req.headers().get_one("User-Agent").unwrap_or("").to_string();
        Outcome::Success(ClientInfo { ip, user_agent })
    }
}

pub fn valida_codigo_autenticador(codigo: &str) -> String {
    let seconds: u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    totp_custom::<Sha1>(
        DEFAULT_STEP,
        6,
        &base32::decode(&codigo.trim().to_lowercase()).unwrap(),
        seconds,
    )
}

// aq é onde que vai receber o json
// e vai fazer a validação do código digitado pela usuerio

#[derive(Debug, Deserialize)]
pub struct EncryptedMfaPayload {
    pub chave_aes_criptografada: String,
    pub iv: String,
    pub mensagem_criptografada: String,
}

#[post("/verifica_mfa", format = "json", data = "<payload>")]
pub async fn vcod(
    payload: Json<EncryptedMfaPayload>,
    cookies: &CookieJar<'_>,
    client: ClientInfo
) -> Result<Json<Option<String>>, Status> {
    // Descriptografa a chave AES
    let chave_privada_pem = crate::chave::obter_chave_privada();
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
    chave_aes_base64.truncate(chave_aes_base64_len);

    let chave_aes_base64_str = String::from_utf8(chave_aes_base64)
        .map_err(|_| Status::BadRequest)?;
    #[allow(deprecated)]
    let chave_aes = base64_decode(&chave_aes_base64_str)
        .map_err(|_| Status::BadRequest)?;

    #[allow(deprecated)]
    let iv = base64_decode(&payload.iv).map_err(|_| Status::BadRequest)?;
    #[allow(deprecated)]
    let mensagem_criptografada = base64_decode(&payload.mensagem_criptografada)
        .map_err(|_| Status::BadRequest)?;

    let decrypted_data = decrypt(
        Cipher::aes_256_cbc(),
        &chave_aes,
        Some(&iv),
        &mensagem_criptografada
    ).map_err(|_| Status::InternalServerError)?;

    let decrypted_json = String::from_utf8(decrypted_data)
        .map_err(|_| Status::BadRequest)?;

    let codigo_mfa: CodigoMfa = serde_json::from_str(&decrypted_json)
        .map_err(|_| Status::BadRequest)?;

    // Continua com a lógica existente usando codigo_mfa.codigo
    let tmp: u64 = 20;
    if let Some(user_id) = cookies.get("user_id") {
        let user_id = user_id.value().parse::<i32>().unwrap();
        let mut conn = conectar_escritor_leitor();
        let usuario = usuarios.find(user_id).first::<Usuario>(&mut conn).ok();

        if let Some(usuario) = usuario {
            let saida_codigo = valida_codigo_autenticador(&usuario.codigo_2fa);

            if codigo_mfa.codigo.trim() == &*saida_codigo {
                let token = sessao::criar_sessao(user_id, tmp, client.ip, client.user_agent);

                let mut cookie = Cookie::new("sessao_token", token.clone());
                let expires = OffsetDateTime::now_utc() + Duration::minutes(tmp.try_into().unwrap());
                cookie.set_expires(expires);
                cookie.set_path("/");
                cookie.set_http_only(true);
                cookie.set_same_site(SameSite::Strict);

                cookies.add(cookie);

                return Ok(Json(Some(token)));
            }
        }
    }
    Ok(Json(None))
}

