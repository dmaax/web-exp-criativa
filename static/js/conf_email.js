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

document.getElementById('verification-form').addEventListener('submit', async function(event) {
    event.preventDefault();
    
    const cpf = document.getElementById('cpf').value.replace(/\D/g, '');
    const codigo = document.getElementById('mfa-code').value;

    try {
        const publicKeyPem = await getPublicKey();
        if (!publicKeyPem) {
            throw new Error("Erro ao obter chave pública");
        }

        // Preparar dados para criptografia
        const mensagemJson = JSON.stringify({ cpf, codigo });

        // Gerar chave AES e IV
        const aesKey = CryptoJS.lib.WordArray.random(32);
        const iv = CryptoJS.lib.WordArray.random(16);

        // Criptografar com AES
        const encrypted = CryptoJS.AES.encrypt(mensagemJson, aesKey, {
            iv: iv,
            mode: CryptoJS.mode.CBC,
            padding: CryptoJS.pad.Pkcs7
        });

        const encryptedMessageBase64 = encrypted.toString();
        const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);
        const ivBase64 = CryptoJS.enc.Base64.stringify(iv);

        // Criptografar chave AES com RSA
        const encryptor = new JSEncrypt();
        encryptor.setPublicKey(publicKeyPem);
        const encryptedAesKey = encryptor.encrypt(aesKeyBase64);

        const payload = {
            chave_aes_criptografada: encryptedAesKey,
            iv: ivBase64,
            mensagem_criptografada: encryptedMessageBase64
        };

        const response = await fetch('/verificaEmailAndCriaContaBanco', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });

        if (response.ok) {
            const result = await response.json();
            if (result === true) {
                window.location.href = '/static/html/login_page.html';
            } else {
                alert('Erro na verificação. Por favor, tente novamente.');
            }
        } else {
            alert('Erro no servidor. Por favor, tente novamente mais tarde.');
        }
    } catch (error) {
        console.error("Erro:", error);
        alert('Erro de conexão. Por favor, tente novamente.');
    }
});
