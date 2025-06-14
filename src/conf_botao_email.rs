use rocket::{http::Status, serde::json::Json};
use rocket::serde::Deserialize;
use crate::autenticador::valida_codigo_autenticador;
use crate::cria_cartao::{cvs_cartao, numeros_cartao};
use crate::login_db::conectar_escritor_leitor;
use crate::models::Usuario;
use crate::schema::usuarios::dsl::*;
use diesel::prelude::*;
use crate::cria_cartao::data_validade_cartao;
use openssl::rsa::Rsa;
use openssl::symm::{decrypt, Cipher};
#[allow(deprecated)]
use base64::{decode as base64_decode};
use crate::chave::obter_chave_privada;
use rocket::serde::json::serde_json;

#[derive(Deserialize)]
pub struct EntradaVerificaMfa {
    pub cpf: String,
    pub codigo: String,
}

#[derive(Deserialize)]
pub struct EncryptedPayload {
    pub chave_aes_criptografada: String,
    pub iv: String,
    pub mensagem_criptografada: String,
}


#[post("/verificaEmailAndCriaContaBanco", format = "json", data = "<payload>")]
pub async fn veri_email_e_cria_conta_usuario_banco(payload: Json<EncryptedPayload>) -> Result<Json<bool>, Status> {
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
    let iv = base64_decode(&payload.iv).map_err(|_| Status::BadRequest)?;
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

    let entrada: EntradaVerificaMfa = serde_json::from_str(&json_descriptografado)
        .map_err(|_| Status::BadRequest)?;

    let mut conn = conectar_escritor_leitor();

    let usuario_result = usuarios
        .filter(cpf.eq(&entrada.cpf))
        .first::<Usuario>(&mut conn)
        .optional();

    if let Ok(Some(usuario)) = usuario_result {
        let saida_codigo = valida_codigo_autenticador(&usuario.codigo_2fa);
        if entrada.codigo.trim() == saida_codigo {
            // criar conta
            let conta_id: i32 = diesel::insert_into(crate::schema::contas::dsl::contas)
                .values((
                    crate::schema::contas::dsl::usuario_id.eq(usuario.id),
                    crate::schema::contas::dsl::saldo.eq("0.00"),
                ))
                .returning(crate::schema::contas::dsl::id)
                .get_result(&mut conn)
                .map_err(|_| Status::InternalServerError)?;

            // criar cartão
            diesel::insert_into(crate::schema::cartoes::dsl::cartoes)
                .values((
                    crate::schema::cartoes::dsl::conta_id.eq(conta_id),
                    crate::schema::cartoes::dsl::numero_cartao.eq(numeros_cartao()),
                    crate::schema::cartoes::dsl::codigo_cartao.eq(cvs_cartao()),
                    crate::schema::cartoes::dsl::data_cartao.eq(data_validade_cartao()),
                    crate::schema::cartoes::dsl::saldo_disponivel.eq("10000.00"),
                    crate::schema::cartoes::dsl::saldo_usado.eq("0.00"),
                ))
                .execute(&mut conn)
                .map_err(|_| Status::InternalServerError)?;

            // criar empréstimo
            diesel::insert_into(crate::schema::emprestimos::dsl::emprestimos)
                .values((
                    crate::schema::emprestimos::dsl::conta_id.eq(conta_id),
                    crate::schema::emprestimos::dsl::valor_disponivel.eq("0.00"),
                    crate::schema::emprestimos::dsl::valor_emprestado.eq("0.00"),
                ))
                .execute(&mut conn)
                .map_err(|_| Status::InternalServerError)?;

            // criar extrato inicial
            diesel::insert_into(crate::schema::extratos::dsl::extratos)
                .values((
                    crate::schema::extratos::dsl::conta_id.eq(conta_id),
                    crate::schema::extratos::dsl::nome_compra.eq("Conta criada"),
                    crate::schema::extratos::dsl::valor.eq("0.00"),
                ))
                .execute(&mut conn)
                .map_err(|_| Status::InternalServerError)?;

            return Ok(Json(true));
        }
    }

    Ok(Json(false))
}