use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::time::{SystemTime, Duration};
use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, Rng};

pub static SESSOES: Lazy<Arc<Mutex<HashMap<String, Sessao>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub struct Sessao {
    pub user_id: i32,
    pub expira_em: SystemTime,
    pub ip: String,
    pub user_agent: String,
}

pub fn gerar_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

// Adicione ip e user_agent como parâmetros
pub fn criar_sessao(user_id: i32, duracao_min: u64, ip: String, user_agent: String) -> String {
    let token = gerar_token();
    let expira_em = SystemTime::now() + Duration::from_secs(duracao_min * 60);
    let sessao = Sessao { user_id, expira_em, ip, user_agent };
    SESSOES.lock().unwrap().insert(token.clone(), sessao);
    token
}

// Valide ip e user_agent
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
