use serde::{Deserialize, Serialize};
use diesel::Queryable;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Cartao {
    pub id: i32,
    pub conta_id: i32,
    pub numero_cartao: String,
    pub data_cartao: String,
    pub codigo_cartao: String,
    pub saldo_disponivel: String,
    pub saldo_usado: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Conta {
    pub id: i32,
    pub usuario_id: i32,
    pub saldo: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Emprestimo {
    pub id: i32,
    pub conta_id: i32,
    pub valor_disponivel: String,
    pub valor_emprestado: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Extrato {
    pub id: i32,
    pub conta_id: i32,
    pub nome_compra: String,
    pub valor: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub email: String,
    pub cpf: String,
    pub data_nascimento: String,
    pub telefone: String,
    pub senha_hash: String,
    pub cep: String,
    pub codigo_2fa: String,
}
