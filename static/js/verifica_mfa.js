async function getPublicKey() {
    try {
        const response = await fetch("/pega-chave");
        const data = await response.json();
        return data.chavepb;
    } catch (error) {
        console.error("Erro ao obter chave pública:", error);
        return null;
    }
}

export async function verificarMFA() {
    const codigo = document.getElementById("codigoMFA").value;
    const resultado = document.getElementById("resultado");

    const publicKeyPem = await getPublicKey();
    if (!publicKeyPem) {
        resultado.textContent = "❌ Erro ao obter chave pública!";
        resultado.style.color = "red";
        return;
    }

    const mensagemJson = JSON.stringify({ codigo });

    // Gera chave AES e IV
    const aesKey = CryptoJS.lib.WordArray.random(32);
    const iv = CryptoJS.lib.WordArray.random(16);

    // Criptografa a mensagem com AES
    const encrypted = CryptoJS.AES.encrypt(mensagemJson, aesKey, {
        iv: iv,
        mode: CryptoJS.mode.CBC,
        padding: CryptoJS.pad.Pkcs7
    });

    const encryptedMessageBase64 = encrypted.toString();
    const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);
    const ivBase64 = CryptoJS.enc.Base64.stringify(iv);

    // Criptografa a chave AES com RSA
    const encryptor = new JSEncrypt();
    encryptor.setPublicKey(publicKeyPem);
    const encryptedAesKey = encryptor.encrypt(aesKeyBase64);

    const payload = {
        chave_aes_criptografada: encryptedAesKey,
        iv: ivBase64,
        mensagem_criptografada: encryptedMessageBase64
    };

    const resposta = await fetch('/verifica_mfa', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    });

    if (!resposta.ok) {
        resultado.textContent = "❌ Erro ao verificar código!";
        resultado.style.color = "red";
        return;
    }

    const token = await resposta.json();

    if (token && token !== null) {
        resultado.textContent = "✅ Código verificado com sucesso!";
        resultado.style.color = "green";
        setTimeout(() => {
            window.location.href = "/static/html/privada/account_page.html";
        }, 1500);
    } else {
        resultado.textContent = "❌ Código inválido!";
        resultado.style.color = "red";
    }
}
