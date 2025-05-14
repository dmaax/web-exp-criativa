use koibumi_base32 as base32;
use std::time::SystemTime;
use totp_lite::{totp_custom, Sha1, DEFAULT_STEP};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{Deserialize, json::Json};
use crate::login_db::conectar_escritor_leitor;
use crate::schema::usuarios::dsl::usuarios;
use crate::models::Usuario;
use crate::sessao;
use diesel::prelude::*;
use rocket::http::Status;


#[derive(Debug, Deserialize)]
pub struct CodigoMfa {
    pub codigo: String,
}
// entra o base32 dai ele vai devolver os 6 numeros
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


#[post("/verifica_mfa", format = "json", data = "<entrada_codigo>")]
pub async fn vcod(entrada_codigo: Json<CodigoMfa>, cookies: &CookieJar<'_>) -> Result<Json<Option<String>>, Status> {
    if let Some(user_id) = cookies.get("user_id") {
        let user_id = user_id.value().parse::<i32>().unwrap();
        let mut conn = conectar_escritor_leitor();
        let usuario = usuarios.find(user_id).first::<Usuario>(&mut conn).ok();

        if let Some(usuario) = usuario {
            let saida_codigo = valida_codigo_autenticador(&usuario.codigo_2fa);
            if entrada_codigo.codigo.trim() == &*saida_codigo {
                let token = sessao::criar_sessao(user_id, 1);
                cookies.add(Cookie::new("sessao_token", token.clone()));

                return Ok(Json(Some(token)));
            }
        }
    }
    Ok(Json(None))
}
