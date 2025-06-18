# Guia Completo de Criptografia - Web Bancária

## Visão Geral
Este guia demonstra como implementar criptografia bidirecional segura entre frontend e backend:
1. Frontend -> Backend: Dados criptografados com AES + chave AES criptografada com RSA
2. Backend -> Frontend: Resposta criptografada com a mesma chave AES

## Frontend (JavaScript)

### 1. Estrutura Básica
```javascript
// Guarda a chave AES atual para descriptografar respostas
let currentAesKey = null;

// Função para descriptografar respostas do servidor
async function decryptResponse(encryptedData, iv, aesKey) {
    const ivParsed = CryptoJS.enc.Base64.parse(iv);
    
    const decrypted = CryptoJS.AES.decrypt(
        encryptedData,
        aesKey,
        {
            iv: ivParsed,
            mode: CryptoJS.mode.CBC,
            padding: CryptoJS.pad.Pkcs7
        }
    );

    return JSON.parse(decrypted.toString(CryptoJS.enc.Utf8));
}
```

### 2. Exemplo de Requisição (Depósito)
```javascript
async function enviarDeposito(valor) {
    try {
        // 1. Gerar nova chave AES e IV
        const aesKey = CryptoJS.lib.WordArray.random(32);
        currentAesKey = aesKey; // Salvar para descriptografar resposta
        const iv = CryptoJS.lib.WordArray.random(16);

        // 2. Criptografar dados com AES
        const mensagemJson = JSON.stringify({ valor });
        const encrypted = CryptoJS.AES.encrypt(mensagemJson, aesKey, {
            iv: iv,
            mode: CryptoJS.mode.CBC,
            padding: CryptoJS.pad.Pkcs7
        });

        // 3. Preparar dados em Base64
        const encryptedMessageBase64 = encrypted.toString();
        const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);
        const ivBase64 = CryptoJS.enc.Base64.stringify(iv);

        // 4. Criptografar chave AES com RSA
        const publicKeyPem = await (await fetch("/pega-chave")).json();
        const encryptor = new JSEncrypt();
        encryptor.setPublicKey(publicKeyPem.chavepb);
        const encryptedAesKey = encryptor.encrypt(aesKeyBase64);

        // 5. Montar payload final
        const payload = {
            chave_aes_criptografada: encryptedAesKey,
            iv: ivBase64,
            mensagem_criptografada: encryptedMessageBase64
        };

        // 6. Enviar requisição
        const resp = await fetch("/depositar", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });

        // 7. Processar resposta criptografada
        const encryptedResponse = await resp.json();
        const dadosDecriptados = await decryptResponse(
            encryptedResponse.encrypted_data,
            encryptedResponse.iv,
            currentAesKey
        );

        return dadosDecriptados;
    } catch (err) {
        console.error("Erro:", err);
        throw err;
    }
}
```

## Backend (Rust)

### 1. Estruturas Básicas
```rust
#[derive(Debug, Deserialize)]
struct EncryptedPayload {
    chave_aes_criptografada: String,
    iv: String,
    mensagem_criptografada: String,
}

#[derive(Serialize)]
struct EncryptedResponse {
    encrypted_data: String,
    iv: String
}
```

### 2. Exemplo de Endpoint (Depósito)
```rust
#[post("/depositar", format = "json", data = "<dados>")]
pub async fn depositar(sessao: SessaoUsuario, dados: Json<Value>) -> Result<Json<EncryptedResponse>, Status> {
    // 1. Extrair payload criptografado
    let payload: EncryptedPayload = match serde_json::from_value(dados.into_inner()) {
        Ok(p) => p,
        Err(_) => return Err(Status::BadRequest),
    };

    // 2. Descriptografar chave AES usando RSA privada
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
    
    // 3. Obter chave AES original
    let chave_aes_base64_str = String::from_utf8(chave_aes_base64)
        .map_err(|_| Status::BadRequest)?;
    let chave_aes = base64.decode(&chave_aes_base64_str)
        .map_err(|_| Status::BadRequest)?;
    
    // 4. Descriptografar dados usando AES
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

    // 5. Processar dados descriptografados
    let decrypted_json = String::from_utf8(decrypted_data)
        .map_err(|_| Status::BadRequest)?;
    let deposito: ValorRequest = serde_json::from_str(&decrypted_json)
        .map_err(|_| Status::BadRequest)?;

    // 6. Lógica de negócio aqui...

    // 7. Criptografar resposta com a mesma chave AES
    let resposta = DadosConta {
        saldo_conta: format!("R$ {}", novo_saldo),
        // ... outros campos
    };

    Ok(Json(resposta.encrypt(&chave_aes)))
}
```

### 3. Implementação da Criptografia de Resposta
```rust
impl DadosConta {
    fn encrypt(&self, key: &[u8]) -> EncryptedResponse {
        // 1. Converter dados para JSON
        let json = serde_json::to_string(&self).unwrap();
        
        // 2. Gerar IV aleatório
        let mut iv = vec![0u8; 16];
        openssl::rand::rand_bytes(&mut iv).unwrap();
        
        // 3. Criptografar com AES
        let encrypted = encrypt(
            Cipher::aes_256_cbc(),
            key,
            Some(&iv),
            json.as_bytes()
        ).unwrap();

        // 4. Retornar resposta em Base64
        EncryptedResponse {
            encrypted_data: base64.encode(encrypted),
            iv: base64.encode(iv)
        }
    }
}
```

## Fluxo Completo
1. Frontend gera chave AES aleatória e IV
2. Frontend criptografa dados com AES
3. Frontend criptografa chave AES com RSA pública
4. Backend descriptografa chave AES com RSA privada
5. Backend descriptografa dados com AES
6. Backend processa dados
7. Backend criptografa resposta com a mesma chave AES
8. Frontend descriptografa resposta com a chave AES guardada

