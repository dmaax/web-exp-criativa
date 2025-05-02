use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use crate::schema::*;


#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "usuarios"]
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

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Usuario)]
#[table_name = "contas"]
pub struct Conta {
    pub id: i32,
    pub usuario_id: i32,
    pub saldo: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Conta)]
#[table_name = "cartoes"]
pub struct Cartao {
    pub id: i32,
    pub conta_id: i32,
    pub numero_cartao: String,
    pub saldo_disponivel: String,
    pub saldo_usado: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Conta)]
#[table_name = "emprestimos"]
pub struct Emprestimo {
    pub id: i32,
    pub conta_id: i32,
    pub valor_disponivel: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Conta)]
#[table_name = "extratos"]
pub struct Extrato {
    pub id: i32,
    pub conta_id: i32,
    pub nome_compra: String,
    pub valor: String,
    pub data_compra: Option<chrono::NaiveDateTime>,
}
