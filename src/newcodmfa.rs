use rand::Rng;
use koibumi_base32::encode;

#[allow(dead_code)]
pub fn gerar_segredo() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.r#gen();
    encode(&bytes)
}


