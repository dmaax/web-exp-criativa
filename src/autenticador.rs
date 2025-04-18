use koibumi_base32 as base32;
use std::time::SystemTime;
use totp_lite::{totp_custom, Sha1, DEFAULT_STEP};
use rocket::http::Status;
use rocket::serde::{Deserialize, json::Json};

#[derive(Debug, Deserialize)]
pub struct CodigoMfa {
    pub codigo: Box<str>,
}

pub fn valida_codigo_autenticador(codigo: &str) -> Box<str> {
    let seconds: u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    totp_custom::<Sha1>(
        DEFAULT_STEP,
        6,
        &base32::decode(&codigo.trim().to_lowercase()).unwrap(),
        seconds,
    ).into()
}

#[post("/verifica_mfa", format = "json", data = "<entrada_codigo>")]
pub async fn vcod(entrada_codigo: Json<CodigoMfa>) -> Result<Json<bool>, Status> {
    let x: &str = "ea273b66in5pvp64sg2gigpwuu";

    let saida_codigo: Box<str> = valida_codigo_autenticador(&x);

    if entrada_codigo.codigo.trim() == &*saida_codigo{
        Ok(Json(true))
    } else {
        Ok(Json(false))
    }
}