use koibumi_base32 as base32;
use std::time::SystemTime;
use totp_lite::{totp_custom, Sha1, DEFAULT_STEP};

pub fn valida_codigo_autenticador(codigo: String) -> String {
    let seconds: u64 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Gera e retorna o código TOTP baseado no segredo e tempo
    totp_custom::<Sha1>(
        DEFAULT_STEP, // Intervalo de 30 segundos
        6,            // Código de 6 dígitos
        &base32::decode(&codigo.trim().to_lowercase()).unwrap(),
        seconds,      // Tempo atual em segundos
    )
}
