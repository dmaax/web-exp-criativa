use diesel::prelude::*;

#[derive(Queryable)]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub email: String,
    pub cpf: String,
    pub data_nascimento: String,
    pub telefone: String,
    pub cep: String,
    pub senha_hash: String,
    pub codigo_2fa: String,
}
