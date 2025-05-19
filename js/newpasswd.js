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
            // Gera os hashes das senhas
            const senhaAtualHash = CryptoJS.SHA256(senhaAtual).toString(CryptoJS.enc.Hex);
            const novaSenhaHash = CryptoJS.SHA256(novaSenha).toString(CryptoJS.enc.Hex);

            console.log("Enviando dados para o servidor:", {
                cpf: cpf,
                senha_atual: senhaAtualHash,
                nova_senha: novaSenhaHash,
            });

            const response = await fetch("/alterar_senha", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    cpf: cpf,
                    senha_atual: senhaAtualHash,
                    nova_senha: novaSenhaHash,
                }),
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
            console.error("Erro ao conectar com o servidor:", error);
            alert("Erro ao conectar com o servidor. Verifique sua conexão ou tente novamente mais tarde.");
        }
    }

    alterarSenhaButton.disabled = false;
    alterarSenhaButton.innerHTML = originalButtonText;
}
