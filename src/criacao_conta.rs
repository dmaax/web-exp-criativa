use rocket::serde::{Deserialize, json::Json};
use rocket::post;
use crate::models::Usuario;
use crate::schema::usuarios::dsl::*;
use diesel::prelude::*;
use crate::login_db::conectar_escritor_leitor;
use crate::mail::{self, send_verification};
use rocket::serde::json::Value;
use openssl::rsa::Rsa;
use openssl::symm::{decrypt, Cipher};
#[allow(deprecated)]
use base64::{decode as base64_decode};
use serde::Deserialize as SerdeDeserialize;
use rocket::serde::json::serde_json;


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

#[derive(Debug, SerdeDeserialize)]
struct EncryptedPayload {
    chave_aes_criptografada: String,
    iv: String,
    mensagem_criptografada: String,
}

#[derive(Debug, SerdeDeserialize)]
#[allow(non_snake_case)]
struct NovoUsuarioDescriptografado {
    nome: String,
    email: String,
    cpf: String,
    dataNascimento: String,
    telefone: String,
    cep: String,
    senhaHash: String,
}


#[post("/entrada_criar_conta", format = "json", data = "<dados>")]
pub fn criar_conta(dados: Json<Value>) -> Json<u8> {
    let payload: EncryptedPayload = match serde_json::from_value(dados.into_inner()) {
        Ok(p) => p,
        Err(_) => return Json(3),
    };

    let chave_privada_pem = std::fs::read("/home/pato/duck2/web-exp-criativa/chave/private_key.pem").expect("Chave privada não encontrada");
    let rsa = Rsa::private_key_from_pem(&chave_privada_pem).expect("Erro ao carregar chave privada");

    #[allow(deprecated)]
    let chave_aes_criptografada = base64_decode(&payload.chave_aes_criptografada).unwrap();
    let mut chave_aes_base64 = vec![0; rsa.size() as usize];
    let chave_aes_base64_len = rsa.private_decrypt(&chave_aes_criptografada, &mut chave_aes_base64, openssl::rsa::Padding::PKCS1).unwrap();
    chave_aes_base64.truncate(chave_aes_base64_len);

    let chave_aes_base64_str = String::from_utf8(chave_aes_base64).unwrap();
    #[allow(deprecated)]
    let chave_aes = base64_decode(&chave_aes_base64_str).unwrap();

    #[allow(deprecated)]
    let iv = base64_decode(&payload.iv).unwrap();

    #[allow(deprecated)]
    let mensagem_criptografada = base64_decode(&payload.mensagem_criptografada).unwrap();
    //println!("Mensagem criptografada: {:?}", mensagem_criptografada);

    let decrypted_data = decrypt(
        Cipher::aes_256_cbc(),
        &chave_aes,
        Some(&iv),
        &mensagem_criptografada
    ).unwrap();

    let decrypted_json = String::from_utf8(decrypted_data).unwrap();
    //println!("Dados descriptografados: {}", decrypted_json);

    let dados: NovoUsuarioDescriptografado = match serde_json::from_str(&decrypted_json) {
        Ok(d) => d,
        Err(_) => return Json(3),
    };

    let mut conn = conectar_escritor_leitor();
    let resultado = usuarios
        .filter(cpf.eq(&dados.cpf))
        .or_filter(email.eq(&dados.email))
        .first::<Usuario>(&mut conn)
        .optional();

    match resultado {
        Ok(Some(_)) => return Json(2),
        Ok(None) => {
            let cod_2fa: String = mail::gerar_segredo();
            let novo_usuario = (
                nome.eq(&dados.nome),
                email.eq(&dados.email),
                cpf.eq(&dados.cpf),
                data_nascimento.eq(&dados.dataNascimento),
                telefone.eq(&dados.telefone),
                cep.eq(&dados.cep),
                senha_hash.eq(&dados.senhaHash),
                codigo_2fa.eq(&cod_2fa),
            );
            let resultado_insercao = diesel::insert_into(usuarios)
                .values(novo_usuario)
                .execute(&mut conn);

            match resultado_insercao {
                Ok(_) => {
                    send_verification(&dados.email, &dados.nome, &cod_2fa);
                    Json(1)
                },
                Err(_) => Json(3),
            }
        },
        Err(_) => Json(3),
    }
}
