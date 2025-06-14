# Guia de Criptografia - Frontend -> Backend

## Visão Geral do Fluxo
1. Frontend obtém chave pública RSA do servidor
2. Frontend cria dados sensíveis em JSON
3. Frontend criptografa usando AES + RSA
4. Backend descriptografa e processa

## Frontend (JavaScript)

### 1. Obter Chave Pública
```javascript
const publicKeyPem = await fetch("/pega-chave").then(r => r.json()).then(d => d.chavepb);
```

### 2. Preparar Dados
```javascript
const dadosSensiveis = { 
    campo1: "valor1",
    campo2: "valor2"
};
const mensagemJson = JSON.stringify(dadosSensiveis);
```

### 3. Criptografar (AES + RSA)
```javascript
// Gerar chave AES e IV aleatórios
const aesKey = CryptoJS.lib.WordArray.random(32); // 256 bits
const iv = CryptoJS.lib.WordArray.random(16);     // 128 bits

// Criptografar dados com AES
const encrypted = CryptoJS.AES.encrypt(mensagemJson, aesKey, {
    iv: iv,
    mode: CryptoJS.mode.CBC,
    padding: CryptoJS.pad.Pkcs7
});

// Converter para Base64
const mensagemCriptografada = encrypted.toString();
const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);
const ivBase64 = CryptoJS.enc.Base64.stringify(iv);

// Criptografar chave AES com RSA
const encryptor = new JSEncrypt();
encryptor.setPublicKey(publicKeyPem);
const chaveAesCriptografada = encryptor.encrypt(aesKeyBase64);

const payload = {
    chave_aes_criptografada: chaveAesCriptografada,
    iv: ivBase64,
    mensagem_criptografada: mensagemCriptografada
};
```

## Backend (Rust)

### 1. Descriptografar Chave AES usando RSA
```rust
let chave_privada_pem = obter_chave_privada();
let rsa = Rsa::private_key_from_pem(&chave_privada_pem.as_bytes())?;

let chave_aes_criptografada = base64_decode(&payload.chave_aes_criptografada)?;
let mut chave_aes_base64 = vec![0; rsa.size() as usize];
let chave_aes_base64_len = rsa.private_decrypt(
    &chave_aes_criptografada,
    &mut chave_aes_base64,
    openssl::rsa::Padding::PKCS1
)?;
```

### 2. Descriptografar Dados usando AES
```rust
let chave_aes = base64_decode(&chave_aes_base64_str)?;
let iv = base64_decode(&payload.iv)?;
let mensagem_criptografada = base64_decode(&payload.mensagem_criptografada)?;

let dados_descriptografados = decrypt(
    Cipher::aes_256_cbc(),
    &chave_aes,
    Some(&iv),
    &mensagem_criptografada
)?;

let json_descriptografado = String::from_utf8(dados_descriptografados)?;
let dados: MeuTipo = serde_json::from_str(&json_descriptografado)?;
```
