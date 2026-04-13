use rocket::serde::{Deserialize, json::Json};
use rocket::post;
use crate::models::Usuario;
use crate::schema::usuarios::dsl::*;
use diesel::prelude::*;
use crate::login_db::conectar_escritor_leitor;
use crate::mail;
use rocket::serde::json::Value;
use crate::autenticador;
use openssl::rsa::Rsa;
use openssl::symm::{decrypt, Cipher};
#[allow(deprecated)]
use base64::{decode as base64_decode};
use serde::Deserialize as SerdeDeserialize;
use rocket::serde::json::serde_json;
use crate::chave::obter_chave_privada;


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

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ValidaMfaConta {
    pub email: String,
    pub cpf: String,
    pub codigo_mfa: String,
}


#[post("/entrada_criar_conta", format = "json", data = "<dados>")]
pub fn criar_conta(dados: Json<Value>) -> Json<Value> {
    let payload: EncryptedPayload = match serde_json::from_value(dados.into_inner()) {
        Ok(p) => p,
        Err(_) => return Json(serde_json::json!({"status": 3})),
    };

    let chave_privada_pem = obter_chave_privada();
    let rsa = Rsa::private_key_from_pem(&chave_privada_pem.as_bytes()).expect("Erro ao carregar chave privada");

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
    println!("Mensagem criptografada: {:?}", mensagem_criptografada);

    let decrypted_data = decrypt(
        Cipher::aes_256_cbc(),
        &chave_aes,
        Some(&iv),
        &mensagem_criptografada
    ).unwrap();

    let decrypted_json = String::from_utf8(decrypted_data).unwrap();
    println!("Dados descriptografados: {}", decrypted_json);

    let dados: NovoUsuarioDescriptografado = match serde_json::from_str(&decrypted_json) {
        Ok(d) => d,
        Err(_) => return Json(serde_json::json!({"status": 3})),
    };

    let mut conn = conectar_escritor_leitor();
    let resultado = usuarios
        .filter(cpf.eq(&dados.cpf))
        .or_filter(email.eq(&dados.email))
        .first::<Usuario>(&mut conn)
        .optional();

    match resultado {
        Ok(Some(_)) => return Json(serde_json::json!({"status": 2})),
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
                    Json(serde_json::json!({
                        "status": 1,
                        "mfa_secret": cod_2fa
                    }))
                },
                Err(_) => Json(serde_json::json!({"status": 3})),
            }
        },
        Err(_) => Json(serde_json::json!({"status": 3})),
    }
}

#[post("/confirma_mfa_conta", format = "json", data = "<dados>")]
pub fn confirma_mfa_conta(dados: Json<ValidaMfaConta>) -> Json<Value> {
    let mut conn = conectar_escritor_leitor();
    
    let resultado = usuarios
        .filter(email.eq(&dados.email))
        .filter(cpf.eq(&dados.cpf))
        .first::<Usuario>(&mut conn)
        .optional();

    match resultado {
        Ok(Some(usuario)) => {
            let codigo_valido = autenticador::valida_codigo_autenticador(&usuario.codigo_2fa);
            
            if codigo_valido == dados.codigo_mfa {
                Json(serde_json::json!({"status": 1, "message": "MFA confirmado com sucesso"}))
            } else {
                Json(serde_json::json!({"status": 2, "message": "Código MFA inválido"}))
            }
        },
        Ok(None) => {
            Json(serde_json::json!({"status": 2, "message": "Usuário não encontrado"}))
        },
        Err(_) => {
            Json(serde_json::json!({"status": 3, "message": "Erro ao confirmar MFA"}))
        }
    }
}

