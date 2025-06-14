function marcarCampoInvalido(idCampo, invalido) {
    const campo = document.getElementById(idCampo);
    if (invalido) {
        campo.classList.add("campo-invalido");
    } else {
        campo.classList.remove("campo-invalido");
    }
}

async function alterarSenha() {
    const alterarSenhaButton = document.getElementById("alterarSenhaButton");
    const originalButtonText = alterarSenhaButton.innerHTML;

    alterarSenhaButton.disabled = true;
    alterarSenhaButton.innerHTML = `<span class="spinner"></span> Processando...`;

    const cpf = document.getElementById("cpf").value.replace(/\D/g, '');
    const senhaAtual = document.getElementById("currentPassword").value.trim();
    const novaSenha = document.getElementById("newPassword").value.trim();
    const confirmarSenha = document.getElementById("confirmPassword").value.trim();
    

    const senhaRegex = /^(?=.*[0-9])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{6,16}$/;

    let cpfValido = cpf.length === 11;
    let senhaAtualValida = senhaAtual !== "";
    let novaSenhaValida = senhaRegex.test(novaSenha);
    let confirmarSenhaValida = novaSenha === confirmarSenha;

    marcarCampoInvalido("cpf", !cpfValido);
    marcarCampoInvalido("currentPassword", !senhaAtualValida);
    marcarCampoInvalido("newPassword", !novaSenhaValida);
    marcarCampoInvalido("confirmPassword", !confirmarSenhaValida);

    if (cpfValido && senhaAtualValida && novaSenhaValida && confirmarSenhaValida) {
        try {
            const senhaAtualHash = CryptoJS.SHA256(senhaAtual).toString(CryptoJS.enc.Hex);
            const novaSenhaHash = CryptoJS.SHA256(novaSenha).toString(CryptoJS.enc.Hex);

            // Obter chave pública
            const publicKeyPem = await getPublicKey();
            if (!publicKeyPem) {
                throw new Error("Erro ao obter chave pública");
            }

            // Preparar dados para criptografia
            const mensagemJson = JSON.stringify({
                cpf: cpf,
                senha_atual: senhaAtualHash,
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

            // Montar payload
            const payload = {
                chave_aes_criptografada: encryptedAesKey,
                iv: ivBase64,
                mensagem_criptografada: encryptedMessageBase64
            };

            const response = await fetch("/alterar_senha", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(payload)
            });

            console.log("Resposta do servidor:", response);

            if (response.ok) {
                const sucesso = await response.json();
                console.log("Resposta JSON do servidor:", sucesso);
                if (sucesso) {
                    alert("Senha alterada com sucesso!");
                    window.location.href = "/static/html/login_page.html";
                } else {
                    alert("CPF ou senha atual incorretos.");
                }
            } else {
                console.error("Erro na resposta do servidor:", response.status, response.statusText);
                alert("Erro ao alterar a senha. Tente novamente mais tarde.");
            }
        } catch (error) {
            console.error("Erro:", error);
            alert("Erro ao processar alteração de senha");
        }
    }

    alterarSenhaButton.disabled = false;
    alterarSenhaButton.innerHTML = originalButtonText;
}

// Adicionar função para obter chave pública
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
