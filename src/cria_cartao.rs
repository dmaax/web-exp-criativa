use rand::Rng;
pub fn numeros_cartao() -> String {
    let mut rng = rand::thread_rng();
    let mut cartao = String::new();
    for _ in 0..16 {
        let numero = rng.gen_range(0..10);
        cartao.push_str(&numero.to_string());
    }
    cartao
}

pub fn cvs_cartao() -> String {
    let mut rng = rand::thread_rng();
    let mut cvs = String::new();
    for _ in 0..3 {
        let numero = rng.gen_range(0..10);
        cvs.push_str(&numero.to_string());
    }
    cvs
}

pub fn data_validade_cartao() -> String {
    let mut rng = rand::thread_rng();
    let mes = rng.gen_range(1..13);
    let ano = rng.gen_range(2026..2035);
    format!("{:02}/{:02}", mes, ano)
}
