use rocket::serde::{Deserialize, json::Json};
use rocket::post;
use crate::models::Usuario;
use crate::schema::usuarios::dsl::*;
use diesel::prelude::*;
use crate::login_db::conectar_escritor_leitor;
use crate::mail::{self, send_verification};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
// para n encher o saco com o nome das variaveis na struct
#[allow(non_snake_case)] 
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
pub fn criar_conta(dados: Json<NovoUsuario>) -> Json<u8> {
    let mut conn = conectar_escritor_leitor(); 
    // procura se o cpf e o email ja estao cadastrados 
    let resultado = usuarios
        // filtra pelo cpf
        .filter(cpf.eq(&dados.cpf)) 
        // filtra pelo email
        .or_filter(email.eq(&dados.email)) 
   // tenta pegar o primeiro usuario que tem o cpf ou o email
        .first::<Usuario>(&mut conn)  
        // retorna em erro se o cpf ou email ja estao cadastrados e um ok se nao estao
        // optional retorna um resultado com o usuario ou None se nao encontrar
        .optional(); 
        
    //aq vamos tratar o resultado em um match
    match resultado { 
         // se o resultado for ok e tiver algum usuario, retorna 2 para o front, e la ele trata esse retorno 
        Ok(Some(_)) => return Json(2),
        // se o resultado for ok e nao tiver nenhum usuario, continua
        Ok(None) => { 
            let cod_2fa: String = mail::gerar_segredo(); // Gera o código 2FA

            // cria uma tupla com os dados do novo usuário
            let novo_usuario = (
                nome.eq(&dados.nome),
                email.eq(&dados.email),
                cpf.eq(&dados.cpf),
                data_nascimento.eq(&dados.dataNascimento),
                telefone.eq(&dados.telefone),
                cep.eq(&dados.cep),
                senha_hash.eq(&dados.senha),
                codigo_2fa.eq(&cod_2fa),
            );

            // insere o novo usuário no banco de dados
            let resultado_insercao = diesel::insert_into(usuarios)
                .values(novo_usuario) 
                // insere o novo usuário (git add por exemplo)
                .execute(&mut conn); 
            // executa a inserção (git push por exemplo)

            match resultado_insercao {
                Ok(_) => {
                    // envia o e-mail de verificação
                    send_verification(&dados.email, &dados.nome, &cod_2fa); 
                    // envia o e-mail de verificação
                    // retorna 1 para o front, que significa que a inserção foi bem sucedida

                    Json(1)
                },
                Err(_) => Json(3), 
                // erro ao inserir no banco de dados
            }
        },
        Err(_) => Json(3), 
        //erro 
    }
}
