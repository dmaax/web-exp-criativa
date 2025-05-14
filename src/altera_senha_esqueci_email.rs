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

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AlterarSenhaEmail {
    pub cpf: String,
    pub mfa: String,
    pub nova_senha: String,
}

fn gerar_codigo_totp(base32_secret: &str) -> Option<String> {
    let seconds: u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()?
        .as_secs();
    let secret = base32::decode(&base32_secret.trim().to_lowercase()).ok()?;
    Some(totp_custom::<Sha1>(DEFAULT_STEP, 6, &secret, seconds))
}

#[post("/alterar_senha_email", format = "json", data = "<dados>")]
pub async fn alterar_senha_email(dados: Json<AlterarSenhaEmail>) -> Result<Json<bool>, Status> {
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
