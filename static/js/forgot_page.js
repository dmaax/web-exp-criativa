document.addEventListener("DOMContentLoaded", () => {
    document.getElementById("send-email").addEventListener("click", async function() {
        const email = document.getElementById("userId").value.trim();
        if (!email) {
            alert("Digite seu e-mail.");
            return;
        }

        try {
            const publicKeyPem = await getPublicKey();
            if (!publicKeyPem) {
                throw new Error("Erro ao obter chave pública");
            }

            // Preparar dados para criptografia
            const mensagemJson = JSON.stringify({ email });

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

            const resp = await fetch("/esqueci_senha", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(payload)
            });
            if (resp.ok) {
                alert("Se o e-mail existir, você receberá instruções para recuperar sua conta.");
                
                window.location.href = "/static/html/login_page.html";

            } else if (resp.status === 404) {
                alert("E-mail não encontrado.");
            } else {
                alert("Erro ao enviar e-mail de recuperação.");
            }
        } catch (error) {
            console.error("Erro:", error);
            alert("Erro ao processar solicitação");
        }
    });
});

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
