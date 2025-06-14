use rocket::{ http::Status, serde::json::Json};
use rocket::{post};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::{contas, extratos};
use crate::login_db::conectar_escritor_leitor;
use crate::SessaoUsuario;
use crate::schema::usuarios;
use openssl::symm::{encrypt, decrypt, Cipher};
use rocket::serde::json::serde_json;
use rocket::serde::json::Value;
use openssl::rsa::Rsa;
use base64::{Engine as _, engine::general_purpose::STANDARD as base64}; // New import for base64

#[derive(Serialize)]
pub struct DadosConta {
    pub saldo_conta: String,

    pub transacoes: Vec<String>,
}
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct DepositoRequest {
    pub valor: f64,
}
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct PagamentoRequest {
    pub valor: f64,
}


#[allow(deprecated)]
#[post("/dados-conta", format = "json", data = "<dados>")]  // Mudando para POST
pub async fn dados_conta(sessao: SessaoUsuario, dados: Json<Value>) -> Result<Json<EncryptedResponse>, Status> {
    let payload: EncryptedPayload = match serde_json::from_value(dados.into_inner()) {
        Ok(p) => p,
        Err(_) => return Err(Status::BadRequest),
    };

    // Descriptografar a chave AES usando RSA
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

    // Resto da lógica existente de buscar dados
    let mut conn = conectar_escritor_leitor();

    // Busca o id da conta do usuário autenticado
    let conta_id_result = contas::dsl::contas
        .inner_join(usuarios::dsl::usuarios.on(contas::dsl::usuario_id.eq(usuarios::dsl::id)))
        .filter(usuarios::dsl::id.eq(sessao.0))
        .select(contas::dsl::id)
        .first::<i32>(&mut conn)
        .optional();

    let conta_id = match conta_id_result {
        Ok(Some(id)) => id,
        _ => return Err(Status::NotFound),
    };

    // Buscar saldo da conta
    let saldo_conta_result: Result<String, _> = contas::dsl::contas
        .filter(contas::dsl::id.eq(conta_id))
        .select(contas::dsl::saldo)
        .first(&mut conn);

    let saldo_conta = saldo_conta_result.unwrap_or_else(|_| "0.00".to_string());

    
    // Buscar transações do extrato
    let extratos_result: Result<Vec<(String, String)>, _> = extratos::dsl::extratos
        .filter(extratos::dsl::conta_id.eq(conta_id))
        .select((extratos::dsl::nome_compra, extratos::dsl::valor))
        .load(&mut conn);

    let transacoes = extratos_result
        .unwrap_or_default()
        .into_iter()
        .map(|(nome, valor)| format!("{} - R$ {}", nome, valor))
        .collect();

    let dados = DadosConta {
        saldo_conta: format!("R$ {}", saldo_conta),
        transacoes,
    };

    // Use a chave AES recebida para criptografar a resposta
    Ok(Json(dados.encrypt(&chave_aes)))
}

#[derive(Debug, Deserialize)]
struct EncryptedPayload {
    chave_aes_criptografada: String,
    iv: String,
    mensagem_criptografada: String,
}

#[derive(Debug, Deserialize)]
struct ValorRequest {
    valor: f64,
}

#[post("/depositar", format = "json", data = "<dados>")]
pub async fn depositar(sessao: SessaoUsuario, dados: Json<Value>) -> Result<Json<EncryptedResponse>, Status> {
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

    let decrypted_json = String::from_utf8(decrypted_data)
        .map_err(|_| Status::BadRequest)?;

    let deposito: ValorRequest = serde_json::from_str(&decrypted_json)
        .map_err(|_| Status::BadRequest)?;

    let mut conn = conectar_escritor_leitor();

    let conta_id = contas::dsl::contas
        .inner_join(usuarios::dsl::usuarios.on(contas::dsl::usuario_id.eq(usuarios::dsl::id)))
        .filter(usuarios::dsl::id.eq(sessao.0))
        .select(contas::dsl::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    // Buscar saldo atual como String
    let saldo_atual_result: Result<String, _> = contas::dsl::contas
        .filter(contas::dsl::id.eq(conta_id))
        .select(contas::dsl::saldo)
        .first(&mut conn);

    // Converter saldo para f64, tratar caso de erro
    let saldo_atual_f64 = match saldo_atual_result {
        Ok(ref s) => s.replace(",", ".").parse::<f64>().unwrap_or(0.0),
        Err(_) => 0.0,
    };

    let novo_saldo_f64 = saldo_atual_f64 + deposito.valor;
    let novo_saldo_str = format!("{:.2}", novo_saldo_f64);

    // Atualizar saldo no banco (como string)
    let update_result = diesel::update(contas::dsl::contas.filter(contas::dsl::id.eq(conta_id)))
        .set(contas::dsl::saldo.eq(&novo_saldo_str))
        .execute(&mut conn);

    if let Err(e) = update_result {
        eprintln!("Erro ao atualizar saldo: {:?}", e);
        return Err(Status::InternalServerError);
    }

    // Adicionar transação no extrato
    let insert_result = diesel::insert_into(extratos::dsl::extratos)
        .values((
            extratos::dsl::conta_id.eq(conta_id),
            extratos::dsl::nome_compra.eq("Depósito"),
            extratos::dsl::valor.eq(format!("{:.2}", deposito.valor)),
        ))
        .execute(&mut conn);

    if let Err(e) = insert_result {
        eprintln!("Erro ao inserir extrato: {:?}", e);
        // Não retorna erro para não travar o fluxo do usuário
    }

    // Buscar extratos atualizados
    let extratos_result: Result<Vec<(String, String)>, _> = extratos::dsl::extratos
        .filter(extratos::dsl::conta_id.eq(conta_id))
        .select((extratos::dsl::nome_compra, extratos::dsl::valor))
        .order(extratos::dsl::id.desc())
        .limit(10)
        .load(&mut conn);

    let transacoes = extratos_result
        .unwrap_or_default()
        .into_iter()
        .map(|(nome, valor)| format!("{} - R$ {}", nome, valor))
        .collect();

    // Create DadosConta struct before encrypting
    let dados_conta = DadosConta {
        saldo_conta: format!("R$ {}", novo_saldo_str),
        transacoes,
    };

    // Use a mesma chave AES para criptografar a resposta
    Ok(Json(dados_conta.encrypt(&chave_aes)))
}

#[post("/pagar-divida", format = "json", data = "<dados>")]
pub async fn pagar_divida(sessao: SessaoUsuario, dados: Json<Value>) -> Result<Json<EncryptedResponse>, Status> {
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

    let decrypted_json = String::from_utf8(decrypted_data)
        .map_err(|_| Status::BadRequest)?;

    let pagamento: ValorRequest = serde_json::from_str(&decrypted_json)
        .map_err(|_| Status::BadRequest)?;

    // Resto da lógica existente
    let mut conn = conectar_escritor_leitor();

    let conta_id = contas::dsl::contas
        .inner_join(usuarios::dsl::usuarios.on(contas::dsl::usuario_id.eq(usuarios::dsl::id)))
        .filter(usuarios::dsl::id.eq(sessao.0))
        .select(contas::dsl::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    // Buscar saldo atual como String
    let saldo_atual_result: Result<String, _> = contas::dsl::contas
        .filter(contas::dsl::id.eq(conta_id))
        .select(contas::dsl::saldo)
        .first(&mut conn);

    // Converter saldo para f64, tratar caso de erro
    let saldo_atual_f64 = match saldo_atual_result {
        Ok(ref s) => s.replace(",", ".").parse::<f64>().unwrap_or(0.0),
        Err(_) => 0.0,
    };

    let novo_saldo_f64 = saldo_atual_f64 - pagamento.valor;
    let novo_saldo_str = format!("{:.2}", novo_saldo_f64);

    // Atualizar saldo no banco (como string)
    let update_result = diesel::update(contas::dsl::contas.filter(contas::dsl::id.eq(conta_id)))
        .set(contas::dsl::saldo.eq(&novo_saldo_str))
        .execute(&mut conn);

    if let Err(e) = update_result {
        eprintln!("Erro ao atualizar saldo: {:?}", e);
        return Err(Status::InternalServerError);
    }

    // Diminuir saldo_usado do cartão (primeiro cartão encontrado da conta)
    use crate::schema::cartoes;
    if let Ok((cartao_id, _conta_id, _numero, _data, _codigo, _limite, saldo_usado)) = cartoes::dsl::cartoes
        .filter(cartoes::dsl::conta_id.eq(conta_id))
        .select((
            cartoes::dsl::id,
            cartoes::dsl::conta_id,
            cartoes::dsl::numero_cartao,
            cartoes::dsl::data_cartao,
            cartoes::dsl::codigo_cartao,
            cartoes::dsl::saldo_disponivel,
            cartoes::dsl::saldo_usado,
        ))
        .first::<(i32, i32, String, String, String, String, String)>(&mut conn)
    {
        let saldo_usado_f = saldo_usado.replace(",", ".").parse::<f64>().unwrap_or(0.0);
        let novo_usado = (saldo_usado_f - pagamento.valor).max(0.0);
        let _ = diesel::update(cartoes::dsl::cartoes.filter(cartoes::dsl::id.eq(cartao_id)))
            .set(cartoes::dsl::saldo_usado.eq(format!("{:.2}", novo_usado)))
            .execute(&mut conn);
    }

    // Adicionar transação no extrato
    let insert_result = diesel::insert_into(extratos::dsl::extratos)
        .values((
            extratos::dsl::conta_id.eq(conta_id),
            extratos::dsl::nome_compra.eq("Pagamento de Dívida"),
            extratos::dsl::valor.eq(format!("-{:.2}", pagamento.valor)),
        ))
        .execute(&mut conn);

    if let Err(e) = insert_result {
        eprintln!("Erro ao inserir extrato: {:?}", e);
    }

    // Buscar extratos atualizados
    let extratos_result: Result<Vec<(String, String)>, _> = extratos::dsl::extratos
        .filter(extratos::dsl::conta_id.eq(conta_id))
        .select((extratos::dsl::nome_compra, extratos::dsl::valor))
        .order(extratos::dsl::id.desc())
        .limit(10)
        .load(&mut conn);

    let transacoes = extratos_result
        .unwrap_or_default()
        .into_iter()
        .map(|(nome, valor)| format!("{} - R$ {}", nome, valor))
        .collect();

    // Create DadosConta struct before encrypting
    let dados_conta = DadosConta {
        saldo_conta: format!("R$ {}", novo_saldo_str),
        transacoes,
    };

    // Use a mesma chave AES para criptografar a resposta
    Ok(Json(dados_conta.encrypt(&chave_aes)))
}

#[derive(Serialize)]
pub struct EncryptedResponse {
    encrypted_data: String,
    iv: String
}
#[allow(deprecated)]
impl DadosConta {
    fn encrypt(&self, key: &[u8]) -> EncryptedResponse {
        let json = serde_json::to_string(&self).unwrap();
        
        // Create a mutable buffer for IV
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
