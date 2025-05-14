use rocket::serde::{Deserialize, json::Json};
use rocket::post;
use rocket::http::Status;
use diesel::prelude::*;
use crate::login_db::conectar_escritor_leitor;
use crate::schema::usuarios::dsl::*;
use crate::models::Usuario;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
// aq oq a gente vai receber
// quando o usuario for alterar a senha
pub struct AlterarSenha {
    pub cpf: String,
    pub senha_atual: String,
    pub nova_senha: String,
}

#[post("/alterar_senha", format = "json", data = "<dados>")]
pub async fn alterar_senha(dados: Json<AlterarSenha>) -> Result<Json<bool>, Status> {
    let mut conn = conectar_escritor_leitor();

    // regex por GALANTIA
    let cpf_limpo = dados.cpf.replace(|c: char| !c.is_numeric(), ""); 

    // busca o usuario pelo CPF
    let usuario_result = usuarios
        .filter(cpf.eq(&cpf_limpo))
        .first::<Usuario>(&mut conn)
        // retorna Ok(None) se não encontrar, o .optional()
        .optional();
    // se achou o usuario, ele vai retornar um Ok(Some(usuario))
    // se não achou, ele vai retornar um Ok(None)
    if let Ok(Some(usuario)) = usuario_result {
        // verifica se o hash da senha atual corresponde ao armazenado
        if dados.senha_atual == usuario.senha_hash {
            // atualiza a senha no banco de dados
            let resultado_atualizacao = diesel::update(usuarios.filter(cpf.eq(&cpf_limpo)))
                .set(senha_hash.eq(&dados.nova_senha))
                .execute(&mut conn);

            if resultado_atualizacao.is_ok() {
                return Ok(Json(true));
            } else {
                return Err(Status::InternalServerError);
            }
        }
    }

    Ok(Json(false))
}
