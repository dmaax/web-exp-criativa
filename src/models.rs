use diesel::prelude::*;
use crate::schema::usuarios;

#[derive(Queryable, Selectable)]
#[diesel(table_name = usuarios)]
#[allow(dead_code)]
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
