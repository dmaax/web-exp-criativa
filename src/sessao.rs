use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::time::{SystemTime, Duration};
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, Rng};

pub static SESSOES: Lazy<Arc<Mutex<HashMap<String, Sessao>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// struct padrao q vms pegar de info
pub struct Sessao {
    pub user_id: i32,
    pub expira_em: SystemTime,
    pub ip: String,
    pub user_agent: String,
}
// aq criamos o token da ssesao
// ele vai ser um string aleatorio de 64 caracteres
pub fn gerar_token() -> String {
    // cria um gerador aleatorios na thread atual 
    rand::thread_rng() 
    // gera bytes alfanuméricos
        .sample_iter(&Alphanumeric)
        // mx 64 caracteres
        .take(64)
    // converte by para string
        .map(char::from)
        // dai junta td e transforma em string
        .collect()
}

// vai chamar o gerar_token
// vai calcular o tempo de expiração
// e vai criar a sessao
// vai inserir a sessao no hashmap 
pub fn criar_sessao(user_id: i32, duracao_min: u64, ip: String, user_agent: String) -> String {
    let token = gerar_token();
    let expira_em = SystemTime::now() + Duration::from_secs(duracao_min * 60);
    let sessao = Sessao { user_id, expira_em, ip, user_agent };
    SESSOES.lock().unwrap().insert(token.clone(), sessao);
    token
}

// aq valida a sessao
// ele vai verificar se o token existe no hashmap
// se existir, ele vai verificar se a sessao ainda é valida
// se a sessao for valida, ele vai verificar se o ip e o user agent batem
// se tudo bater, ele vai retornar o id do usuario
// se a sessao não for valida, ele vai remover a sessao do hashmap

pub fn validar_sessao(token: &str, ip: &str, user_agent: &str) -> Option<i32> {
    let mut sessoes = SESSOES.lock().unwrap();
    if let Some(sessao) = sessoes.get(token) {
        if sessao.expira_em > SystemTime::now()
            && sessao.ip == ip
            && sessao.user_agent == user_agent
        {
            return Some(sessao.user_id);
        } else if sessao.expira_em <= SystemTime::now() {
            sessoes.remove(token);
        }
    }
    None
}
