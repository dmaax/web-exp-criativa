use rand::Rng;
pub fn cria_numero()-> String {
    let mut rng = rand::thread_rng();
    let mut numero_cartao = String::new();
    for _ in 0..16 {
        let digito: u8 = rng.gen_range(0..10);
        numero_cartao.push_str(&digito.to_string());
    }
    numero_cartao
}