use serde::Deserialize;

// 1. Struct para receber o payload criptografado
#[derive(Debug, Deserialize)]
pub struct EncryptedPayload {
    pub chave_aes_criptografada: String,  // Chave AES criptografada com RSA
    pub iv: String,                       // Vetor de inicialização para AES
    pub mensagem_criptografada: String,   // Dados criptografados com AES
}

// 2. Struct para os dados após descriptografia (exemplo login)
#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub senha: String,
}

// 3. Struct para os dados após descriptografia (exemplo MFA)
#[derive(Debug, Deserialize)]
pub struct MfaData {
    pub codigo: String,
}

// 4. Exemplo de implementação da rota
pub fn exemplo_rota(payload: Json<EncryptedPayload>) -> Result<Json<bool>, Status> {
    // A. Descriptografar a chave AES usando RSA
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

    // B. Preparar chave AES e IV
    let chave_aes_base64_str = String::from_utf8(chave_aes_base64[..chave_aes_base64_len].to_vec())
        .map_err(|_| Status::BadRequest)?;
        
    #[allow(deprecated)]
    let chave_aes = base64_decode(&chave_aes_base64_str)
        .map_err(|_| Status::BadRequest)?;
    
    #[allow(deprecated)]
    let iv = base64_decode(&payload.iv)
        .map_err(|_| Status::BadRequest)?;
    
    // C. Descriptografar a mensagem usando AES
    #[allow(deprecated)]
    let mensagem_criptografada = base64_decode(&payload.mensagem_criptografada)
        .map_err(|_| Status::BadRequest)?;

    let dados_descriptografados = decrypt(
        Cipher::aes_256_cbc(),
        &chave_aes,
        Some(&iv),
        &mensagem_criptografada
    ).map_err(|_| Status::InternalServerError)?;

    // D. Converter para JSON e deserializar
    let json_descriptografado = String::from_utf8(dados_descriptografados)
        .map_err(|_| Status::BadRequest)?;

    // E. Exemplo com LoginData
    let dados_login: LoginData = serde_json::from_str(&json_descriptografado)
        .map_err(|_| Status::BadRequest)?;

    // Usar dados_login.email e dados_login.senha
    Ok(Json(true))
}

// 5. Exemplo de teste
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estruturas() {
        // Exemplo de payload recebido
        let payload = EncryptedPayload {
            chave_aes_criptografada: "BASE64_DA_CHAVE_AES_CRIPTOGRAFADA".to_string(),
            iv: "BASE64_DO_IV".to_string(),
            mensagem_criptografada: "BASE64_DA_MENSAGEM".to_string(),
        };

        // Exemplo de dados de login
        let login = LoginData {
            email: "teste@email.com".to_string(),
            senha: "senha_hash".to_string(),
        };

        // Exemplo de dados MFA
        let mfa = MfaData {
            codigo: "123456".to_string(),
        };

        assert_eq!(payload.iv.is_empty(), false);
        assert_eq!(login.email, "teste@email.com");
        assert_eq!(mfa.codigo, "123456");
    }
}
