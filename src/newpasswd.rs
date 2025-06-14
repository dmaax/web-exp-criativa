use rocket::serde::{Deserialize, json::Json};
use rocket::post;
use rocket::http::Status;
use diesel::prelude::*;
use crate::login_db::conectar_escritor_leitor;
use crate::schema::usuarios::dsl::*;
use crate::models::Usuario;
use openssl::rsa::Rsa;
use openssl::symm::{decrypt, Cipher};
#[allow(deprecated)]
use base64::{decode as base64_decode};
use rocket::serde::json::serde_json;
use crate::chave::obter_chave_privada;

#[derive(Debug, Deserialize)]
pub struct EncryptedPayload {
    pub chave_aes_criptografada: String,
    pub iv: String,
    pub mensagem_criptografada: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
// aq oq a gente vai receber
// quando o usuario for alterar a senha
pub struct AlterarSenha {
    pub cpf: String,
    pub senha_atual: String,
    pub nova_senha: String,
}

#[post("/alterar_senha", format = "json", data = "<payload>")]
pub async fn alterar_senha(payload: Json<EncryptedPayload>) -> Result<Json<bool>, Status> {
    // Descriptografa a chave AES
    let chave_privada_pem = obter_chave_privada();
    let rsa = Rsa::private_key_from_pem(&chave_privada_pem.as_bytes())
        .map_err(|_| Status::InternalServerError)?;

    #[allow(deprecated)]
    let chave_aes_criptografada = base64_decode(&payload.chave_aes_criptografada)
        .map_err(|_| Status::BadRequest)?;
    let mut chave_aes_base64 = vec![0; rsa.size() as usize];
    let chave_aes_base64_len = rsa.private_decrypt(
        &chave_aes_criptografada,
        &mut chave_aes_base64,
        openssl::rsa::Padding::PKCS1
    ).map_err(|_| Status::InternalServerError)?;

    let chave_aes_base64_str = String::from_utf8(chave_aes_base64[..chave_aes_base64_len].to_vec())
        .map_err(|_| Status::BadRequest)?;
    #[allow(deprecated)]
    let chave_aes = base64_decode(&chave_aes_base64_str)
        .map_err(|_| Status::BadRequest)?;

    #[allow(deprecated)]
    let iv = base64_decode(&payload.iv)
        .map_err(|_| Status::BadRequest)?;
    #[allow(deprecated)]
    let mensagem_criptografada = base64_decode(&payload.mensagem_criptografada)
        .map_err(|_| Status::BadRequest)?;

    let dados_descriptografados = decrypt(
        Cipher::aes_256_cbc(),
        &chave_aes,
        Some(&iv),
        &mensagem_criptografada
    ).map_err(|_| Status::InternalServerError)?;

    let json_descriptografado = String::from_utf8(dados_descriptografados)
        .map_err(|_| Status::BadRequest)?;

    let dados: AlterarSenha = serde_json::from_str(&json_descriptografado)
        .map_err(|_| Status::BadRequest)?;

    let mut conn = conectar_escritor_leitor();

    // regex por GALANTIA
    let cpf_limpo = dados.cpf.replace(|c: char| !c.is_numeric(), ""); 

    // busca o usuario pelo CPF
    let usuario_result = usuarios
        .filter(cpf.eq(&cpf_limpo))
        .first::<Usuario>(&mut conn)
        // retorna ok(None) se não encontrar, o .optional()
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
