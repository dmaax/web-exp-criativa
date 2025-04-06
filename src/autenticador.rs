use koibumi_base32 as base32;
use std::time::SystemTime;
use totp_lite::{totp_custom, Sha1, DEFAULT_STEP};

pub fn valida_codigo_autenticador(codigo: String) -> String {
    let seconds: u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // gera o cod
    totp_custom::<Sha1>(
        DEFAULT_STEP, //30 seg
        6,            // Codigo de 6 DIGIIITU
        &base32::decode(&codigo.trim().to_lowercase()).unwrap(),
        seconds,      // tempo atual em segundos
    )
}
