use rand::Rng;
use koibumi_base32::encode;

pub fn gerar_segredo() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.r#gen();  // Gera 16 bytes aleatórios
    encode(&bytes)  // Codifica esses bytes em base32
}


