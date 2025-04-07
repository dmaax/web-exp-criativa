use koibumi_base32 as base32;
use std::time::SystemTime;
use totp_lite::{totp_custom, Sha1, DEFAULT_STEP};
use rocket::http::Status;
use rocket::serde::{Deserialize, json::Json};

#[derive(Debug, Deserialize)]
pub struct Codigo_MFA {
    pub codigo: String,
}

pub fn valida_codigo_autenticador(codigo: String) -> String {
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

#[post("/verifica_mfa", format = "json", data = "<entrada_codigo>")]
pub async fn vcod(entrada_codigo: Json<Codigo_MFA>) -> Result<String, Status> {
    let x = "ea273b66in5pvp64sg2gigpwuu".to_string();

    let saida_codigo: String = valida_codigo_autenticador(x);

    if entrada_codigo.codigo.trim() == saida_codigo {
        Ok("true".to_string())
    } else {
        Ok("false".to_string())
    }
}