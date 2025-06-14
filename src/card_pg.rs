use rocket::{post, http::Status, serde::json::Json};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::{cartoes, extratos, contas, usuarios};
use crate::login_db::conectar_escritor_leitor;
use crate::SessaoUsuario;
use openssl::symm::{encrypt, decrypt, Cipher};
use rocket::serde::json::Value;
use openssl::rsa::Rsa;
use base64::{Engine as _, engine::general_purpose::STANDARD as base64};
use rocket::serde::json::serde_json;

#[derive(Serialize)]
pub struct CartaoInfo {
    pub id: i32,
    pub label: String,
    pub numero: String,
    pub data_cartao: String,
    pub codigo_cartao: String,
    pub limite: String,
    pub usado: String,
}

#[derive(Debug, Deserialize)]
pub struct EncryptedPayload {
    chave_aes_criptografada: String,
    iv: String,
    mensagem_criptografada: String,
}

#[derive(Serialize)]
pub struct EncryptedResponse {
    encrypted_data: String,
    iv: String
}

impl CartaoInfo {
    fn encrypt(&self, key: &[u8]) -> EncryptedResponse {
        let json = serde_json::to_string(&self).unwrap();
        let mut iv = vec![0u8; 16];
        openssl::rand::rand_bytes(&mut iv).unwrap();
        
        let encrypted = encrypt(
            Cipher::aes_256_cbc(),
            key,
            Some(&iv),
            json.as_bytes()
        ).unwrap();

        EncryptedResponse {
            encrypted_data: base64.encode(encrypted),
            iv: base64.encode(iv)
        }
    }
}

#[post("/cartoes", format = "json", data = "<dados>")]
pub async fn listar_cartoes(sessao: SessaoUsuario, dados: Json<Value>) -> Result<Json<EncryptedResponse>, Status> {
    let payload: EncryptedPayload = match serde_json::from_value(dados.into_inner()) {
        Ok(p) => p,
        Err(_) => return Err(Status::BadRequest),
    };

    // Descriptografar usando a chave privada RSA
    let chave_privada_pem = crate::chave::obter_chave_privada();
    let rsa = Rsa::private_key_from_pem(chave_privada_pem.as_bytes())
        .map_err(|_| Status::InternalServerError)?;

    let chave_aes_criptografada = base64.decode(&payload.chave_aes_criptografada)
        .map_err(|_| Status::BadRequest)?;
    
    let mut chave_aes_base64 = vec![0; rsa.size() as usize];
    let chave_aes_base64_len = rsa.private_decrypt(
        &chave_aes_criptografada,
        &mut chave_aes_base64,
        openssl::rsa::Padding::PKCS1
    ).map_err(|_| Status::InternalServerError)?;
    
    chave_aes_base64.truncate(chave_aes_base64_len);
    
    let chave_aes_base64_str = String::from_utf8(chave_aes_base64)
        .map_err(|_| Status::BadRequest)?;
    
    let chave_aes = base64.decode(&chave_aes_base64_str)
        .map_err(|_| Status::BadRequest)?;

    // Descriptografar dados (se houver)
    let iv = base64.decode(&payload.iv)
        .map_err(|_| Status::BadRequest)?;
    
    let mensagem_criptografada = base64.decode(&payload.mensagem_criptografada)
        .map_err(|_| Status::BadRequest)?;

    if !mensagem_criptografada.is_empty() {
        let decrypted_data = decrypt(
            Cipher::aes_256_cbc(),
            &chave_aes,
            Some(&iv),
            &mensagem_criptografada
        ).map_err(|_| Status::InternalServerError)?;

        let decrypted_str = String::from_utf8(decrypted_data)
            .map_err(|_| Status::BadRequest)?;

        // Process decrypted data if needed
        println!("Dados descriptografados: {}", decrypted_str);
    }

    let mut conn = conectar_escritor_leitor();
    // Busca o id da conta do usuário autenticado
    let conta_id = contas::dsl::contas
        .inner_join(usuarios::dsl::usuarios.on(contas::dsl::usuario_id.eq(usuarios::dsl::id)))
        .filter(usuarios::dsl::id.eq(sessao.0))
        .select(contas::dsl::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    let result = cartoes::dsl::cartoes
        .filter(cartoes::dsl::conta_id.eq(conta_id))
        .first::<(i32, i32, String, String, String, String, String)>(&mut conn);

    match result {
        Ok((id, _conta_id, numero, data_cartao, codigo_cartao, saldo_disp, saldo_usado)) => {
            let cartao = CartaoInfo {
                id,
                label: format!("Cartão {}", &numero[numero.len().saturating_sub(4)..]),
                numero,
                data_cartao,
                codigo_cartao,
                limite: format!("R$ {}", saldo_disp),
                usado: format!("R$ {}", saldo_usado),
            };
            Ok(Json(cartao.encrypt(&chave_aes)))
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[derive(Debug, Deserialize)]
struct CompraRequest {
    pub cartao_id: i32,
    pub nome_compra: String,
    pub valor: f64,
}

#[post("/compra", format = "json", data = "<dados>")]
pub async fn registrar_compra(sessao: SessaoUsuario, dados: Json<Value>) -> Result<Json<EncryptedResponse>, Status> {
    let payload: EncryptedPayload = match serde_json::from_value(dados.into_inner()) {
        Ok(p) => p,
        Err(_) => return Err(Status::BadRequest),
    };

    // Descriptografar usando a chave privada RSA
    let chave_privada_pem = crate::chave::obter_chave_privada();
    let rsa = Rsa::private_key_from_pem(chave_privada_pem.as_bytes())
        .map_err(|_| Status::InternalServerError)?;

    let chave_aes_criptografada = base64.decode(&payload.chave_aes_criptografada)
        .map_err(|_| Status::BadRequest)?;
    
    let mut chave_aes_base64 = vec![0; rsa.size() as usize];
    let chave_aes_base64_len = rsa.private_decrypt(
        &chave_aes_criptografada,
        &mut chave_aes_base64,
        openssl::rsa::Padding::PKCS1
    ).map_err(|_| Status::InternalServerError)?;
    
    chave_aes_base64.truncate(chave_aes_base64_len);
    
    let chave_aes_base64_str = String::from_utf8(chave_aes_base64)
        .map_err(|_| Status::BadRequest)?;
    
    let chave_aes = base64.decode(&chave_aes_base64_str)
        .map_err(|_| Status::BadRequest)?;

    let iv = base64.decode(&payload.iv)
        .map_err(|_| Status::BadRequest)?;
    
    let mensagem_criptografada = base64.decode(&payload.mensagem_criptografada)
        .map_err(|_| Status::BadRequest)?;

    let decrypted_data = decrypt(
        Cipher::aes_256_cbc(),
        &chave_aes,
        Some(&iv),
        &mensagem_criptografada
    ).map_err(|_| Status::InternalServerError)?;

    let decrypted_str = String::from_utf8(decrypted_data)
        .map_err(|_| Status::BadRequest)?;

    let compra: CompraRequest = serde_json::from_str(&decrypted_str)
        .map_err(|_| Status::BadRequest)?;

    let mut conn = conectar_escritor_leitor();

    // Busca o id da conta do usuário autenticado
    let conta_id = contas::dsl::contas
        .inner_join(usuarios::dsl::usuarios.on(contas::dsl::usuario_id.eq(usuarios::dsl::id)))
        .filter(usuarios::dsl::id.eq(sessao.0))
        .select(contas::dsl::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    // Buscar saldo disponível e usado do cartão do usuário
    let cartao = cartoes::dsl::cartoes
        .filter(cartoes::dsl::id.eq(compra.cartao_id))
        .filter(cartoes::dsl::conta_id.eq(conta_id))
        .first::<(i32, i32, String, String, String, String, String)>(&mut conn)
        .ok();

    if let Some((_id, _conta_id, _numero, _data, _codigo, saldo_disp, saldo_usado)) = cartao {
        let saldo_disp_f = saldo_disp.replace(",", ".").parse::<f64>().unwrap_or(0.0);
        let saldo_usado_f = saldo_usado.replace(",", ".").parse::<f64>().unwrap_or(0.0);

        if compra.valor > (saldo_disp_f - saldo_usado_f) {
            return Err(Status::BadRequest);
        }

        let novo_usado = saldo_usado_f + compra.valor;

        // Atualiza saldo usado
        let _ = diesel::update(cartoes::dsl::cartoes.filter(cartoes::dsl::id.eq(compra.cartao_id)))
            .set(cartoes::dsl::saldo_usado.eq(format!("{:.2}", novo_usado)))
            .execute(&mut conn);

        // Adiciona no extrato
        let _ = diesel::insert_into(extratos::dsl::extratos)
            .values((
                extratos::dsl::conta_id.eq(conta_id),
                extratos::dsl::nome_compra.eq(&compra.nome_compra),
                extratos::dsl::valor.eq(format!("{:.2}", compra.valor)),
            ))
            .execute(&mut conn);

        let cartao_atualizado = CartaoInfo {
            id: _id,
            label: format!("Cartão {}", &_numero[_numero.len().saturating_sub(4)..]),
            numero: _numero,
            data_cartao: _data,
            codigo_cartao: _codigo,
            limite: format!("R$ {}", saldo_disp),
            usado: format!("R$ {}", novo_usado),
        };

        Ok(Json(cartao_atualizado.encrypt(&chave_aes)))
    } else {
        Err(Status::NotFound)
    }
}
