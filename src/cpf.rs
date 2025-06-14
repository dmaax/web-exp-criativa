// https://github.com/andrelmlins/cpf_cnpj

use rocket::{http::Status, serde::json::Json};
use openssl::rsa::Rsa;
use openssl::symm::{decrypt, Cipher};
#[allow(deprecated)]
use base64::{decode as base64_decode};
use crate::chave::obter_chave_privada;
use rocket::serde::json::serde_json;

#[derive(serde::Deserialize)]
pub struct EncryptedPayload {
    pub chave_aes_criptografada: String,
    pub iv: String,
    pub mensagem_criptografada: String,
}

#[derive(serde::Deserialize)]
pub struct CpfConf {
    pub cpf: String,
}

#[derive(serde::Serialize)]
pub struct CpfResponse {
    pub valido: bool,
}

pub fn validate(valor: &str) -> bool {
    let numbers = valor
        .chars()
        .filter(|s| !"./-".contains(s.to_owned()))
        .collect::<Vec<_>>();

    if numbers.len() != 11 || equal_digits(&numbers) {
        return false; 
    }
    let digit_one = validate_first_digit(&numbers);
    if digit_one != numbers[9].to_string().parse::<usize>().unwrap() {
        return false;
    }
    let digit_second = validate_second_digit(&numbers);
    if digit_second != numbers[10].to_string().parse::<usize>().unwrap() {
        return false;
    }
    return true;
}

fn validate_first_digit(numbers: &Vec<char>) -> usize {
    let mut count = 10;
    let mut _sum = 0;
    for number in numbers {
        if count >= 2 {
            let number_int = number.to_string().parse::<usize>().unwrap();
            _sum += number_int * count;
            count -= 1;
        }
    }
    let result = (_sum * 10) % 11;

    if result == 10 {
        return 0;
    } else {
        return result;
    }
}

fn validate_second_digit(numbers: &Vec<char>) -> usize {
    let mut count = 11;
    let mut _sum = 0;
    for number in numbers {
        if count >= 2 {
            let number_int = number.to_string().parse::<usize>().unwrap();
            _sum += number_int * count;
            count -= 1;
        }
    }
    let result = (_sum * 10) % 11;

    if result == 10 {
        return 0;
    } else {
        return result;
    }
}
fn equal_digits(numbers: &Vec<char>) -> bool {
    let mut digit = String::from("");
    for number in numbers {
        if digit == "" {
            digit = number.to_string();
        } else if digit != number.to_string() {
            return false;
        }
    }
    return true;
}

// simples, recebe o cpf e devolve se é valido ou nao
#[post("/verifica_cpf", format = "json", data = "<payload>")]
pub async fn vcpf(payload: Json<EncryptedPayload>) -> Result<Json<CpfResponse>, Status> {
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

    let dados: CpfConf = serde_json::from_str(&json_descriptografado)
        .map_err(|_| Status::BadRequest)?;

    let valido: bool = validate(&dados.cpf);
    Ok(Json(CpfResponse { valido }))
}
