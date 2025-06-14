function marcarCampoInvalido(idCampo, invalido) {
    const campo = document.getElementById(idCampo);
    if (invalido) {
        campo.classList.add("campo-invalido");
    } else {
        campo.classList.remove("campo-invalido");
    }
}

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

async function alterarSenha() {
    const alterarSenhaButton = document.getElementById("alterarSenhaButton");
    const originalButtonText = alterarSenhaButton.innerHTML;

    alterarSenhaButton.disabled = true;
    alterarSenhaButton.innerHTML = `<span class="spinner"></span> Processando...`;

    const cpf = document.getElementById("cpf").value.replace(/\D/g, '');
    const mfa = document.getElementById("mfa").value.trim();
    const novaSenha = document.getElementById("newPassword").value.trim();
    const confirmarSenha = document.getElementById("confirmPassword").value.trim();

    const senhaRegex = /^(?=.*[0-9])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{6,16}$/;

    let cpfValido = cpf.length === 11;
    let mfaValido = mfa.length === 6 && /^\d+$/.test(mfa);
    let novaSenhaValida = senhaRegex.test(novaSenha);
    let confirmarSenhaValida = novaSenha === confirmarSenha;

    
    marcarCampoInvalido("cpf", !cpfValido);
    marcarCampoInvalido("mfa", !mfaValido);
    marcarCampoInvalido("newPassword", !novaSenhaValida);
    marcarCampoInvalido("confirmPassword", !confirmarSenhaValida);

    if (cpfValido && mfaValido && novaSenhaValida && confirmarSenhaValida) {
        try {
            const novaSenhaHash = CryptoJS.SHA256(novaSenha).toString(CryptoJS.enc.Hex);
            
            const publicKeyPem = await getPublicKey();
            if (!publicKeyPem) {
                throw new Error("Erro ao obter chave pública");
            }

            const mensagemJson = JSON.stringify({
                cpf: cpf,
                mfa: mfa,
                nova_senha: novaSenhaHash,
            });

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

            const response = await fetch("/alterar_senha_email", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(payload)
            });

            console.log("Resposta do servidor:", response);

            if (response.ok) {
                const sucesso = await response.json();
                console.log("Resposta JSON do servidor:", sucesso);
                if (sucesso) {
                    window.location.href = "/static/html/login_page.html";
                } else {
                }
            } else {
            }
        } catch (error) {
            console.error("Erro ao conectar com o servidor:", error);
            alert("Erro ao processar alteração de senha");
        }
    }

    alterarSenhaButton.disabled = false;
    alterarSenhaButton.innerHTML = originalButtonText;
}
