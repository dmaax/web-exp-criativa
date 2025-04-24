use rocket::serde::{Deserialize, json::Json};
use rocket::post;
use crate::models::Usuario;
use crate::schema::usuarios::dsl::*;
use diesel::prelude::*;
use crate::login_db::conectar_escritor;
use crate::mail::send_verification;


#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NovoUsuario {
    pub nome: String,
    pub email: String,
    pub cpf: String,
    pub dataNascimento: String,
    pub telefone: String,
    pub cep: String,
    pub senha: String,
}

#[post("/entrada_criar_conta", format = "json", data = "<dados>")]
pub fn criar_conta(dados: Json<NovoUsuario>) -> Json<i32> {
    let mut conn = conectar_escritor(); 

    // Verifica se o usuário já existe no banco de dados (por CPF ou e-mail)
    let resultado = usuarios
        .filter(cpf.eq(&dados.cpf))
        .or_filter(email.eq(&dados.email))
        .first::<Usuario>(&mut conn)
        .optional();

    match resultado {
        Ok(Some(_)) => return Json(2),
        Ok(None) => {
            // Gera código 2FA fixo ou com lib de 2FA
            let cod_2fa = "ea273b66in5pvp64sg2gigpwuu";

            // Criação do novo usuário no banco
            let novo_usuario = (
                nome.eq(&dados.nome),
                email.eq(&dados.email),
                cpf.eq(&dados.cpf),
                data_nascimento.eq(&dados.dataNascimento),
                telefone.eq(&dados.telefone),
                cep.eq(&dados.cep),
                senha_hash.eq(&dados.senha),
                codigo_2fa.eq(cod_2fa),
            );

            // Insere o novo usuário no banco de dados
            let resultado_insercao = diesel::insert_into(usuarios)
                .values(novo_usuario)
                .execute(&mut conn);

            match resultado_insercao {
                Ok(_) => {
                    // Envia o e-mail de verificação
                    send_verification(dados.email.clone(), dados.nome.clone());

                    Json(1)
                },
                Err(_) => Json(3),
            }
        },
        Err(_) => Json(3),
    }
}
