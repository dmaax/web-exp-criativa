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
// a gente precisa disso para o diesel saber como vai fazer a inserção no banco de dados, como sao os dados
// dai o dielsel vai saber como converter codigo rust para sql dieetamente