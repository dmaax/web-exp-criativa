document.getElementById("login-tab").addEventListener("click", function () {
    window.location.href = "/static/html/login_page.html";
});

function verificaemail(e) {
    const regexEmailPucpr = /^[a-zA-Z0-9._%+-]+@pucpr\.edu\.br$/;
    return regexEmailPucpr.test(e);
}

function verificacpfbasico(c) {
    return c.length === 11;
}

async function verificacpfbkend(c2) {
    try {
        const publicKeyPem = await getPublicKey();
        if (!publicKeyPem) {
            console.error("Erro ao obter chave pública");
            return false;
        }

        const mensagemJson = JSON.stringify({ cpf: c2 });

        // Gerar chave AES e IV
        const aesKey = CryptoJS.lib.WordArray.random(32);
        const iv = CryptoJS.lib.WordArray.random(16);

        // Criptografar mensagem com AES
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

        const response = await fetch('/verifica_cpf', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });

        if (!response.ok) return false;
        const data = await response.json();
        return data.valido;
    } catch (error) {
        console.error("Erro na verificação do CPF:", error);
        return false;
    }
}

function verificaidade(i) {
    let dataNasc = new Date(i);
    let hoje = new Date();
    let idade = hoje.getFullYear() - dataNasc.getFullYear();
    let mesAtual = hoje.getMonth(), diaAtual = hoje.getDate();
    let mesNasc = dataNasc.getMonth(), diaNasc = dataNasc.getDate();
    if (mesAtual < mesNasc || (mesAtual === mesNasc && diaAtual < diaNasc)) idade--;
    return idade >= 18;
}

function resultadosenha(s1, s2) {
    const regex = /^(?=.*[0-9])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{6,16}$/;
    if (s1 === s2 && regex.test(s1)) {
        const hash = CryptoJS.SHA256(s1).toString(CryptoJS.enc.Hex);
        return { valido: true, hash };
    }
    return { valido: false, hash: null };
}

function resultadotelefone(tel) {
    return /^\d{10,11}$/.test(tel);
}

function resultadocep(cp) {
    return /^\d{5}-?\d{3}$/.test(cp);
}

function marcarCampoInvalido(idCampo, invalido) {
    const campo = document.getElementById(idCampo);
    campo.classList.toggle("campo-invalido", invalido);
}

async function getPublicKey() {
    try {
        const response = await fetch("/pega-chave");
        const data = await response.json();
        console.log("Chave pública recebida:", data);
        return data.chavepb;
    } catch (error) {
        console.error("Erro ao obter chave pública:", error);
        return null;
    }
}


async function validarCadastro() {
    const cadastrarButton = document.querySelector(".register-button");
    const originalButtonText = cadastrarButton.innerHTML;
    cadastrarButton.disabled = true;
    cadastrarButton.innerHTML = `<span class="spinner"></span> Processando...`;

    let nome = document.getElementById("iname").value;
    let email = document.getElementById("iemail").value;
    let cpf = document.getElementById("icpf").value.replace(/\D/g, '');
    let dataNascimento = document.getElementById("ibirthdate").value;
    let telefone = document.getElementById("icellphone").value.replace(/\D/g, '');
    let senha = document.getElementById("password").value;
    let confirmarSenha = document.getElementById("confirmPassword").value;
    let cep = document.getElementById("icep").value.replace(/\D/g, '');

    let resultadoemail = verificaemail(email);
    let resultadocpfbasico = verificacpfbasico(cpf);
    let resultadocpfbkend = resultadocpfbasico ? await verificacpfbkend(cpf) : false;
    let resultadoidade = verificaidade(dataNascimento);
    let senhaResultado = resultadosenha(senha, confirmarSenha);
    let resultadotelefoneok = resultadotelefone(telefone);
    let resultadocepok = resultadocep(cep);

    marcarCampoInvalido("iemail", !resultadoemail);
    marcarCampoInvalido("icpf", !(resultadocpfbasico && resultadocpfbkend));
    marcarCampoInvalido("ibirthdate", !resultadoidade);
    marcarCampoInvalido("password", !senhaResultado.valido);
    marcarCampoInvalido("confirmPassword", !senhaResultado.valido);
    marcarCampoInvalido("icellphone", !resultadotelefoneok);
    marcarCampoInvalido("icep", !resultadocepok);

    if (
        resultadoemail && resultadocpfbasico && resultadocpfbkend &&
        resultadoidade && senhaResultado.valido &&
        resultadotelefoneok && resultadocepok
    ) {
        const publicKeyPem = await getPublicKey();
        if (!publicKeyPem) {
            alert("Erro ao obter a chave pública do servidor.");
            return;
        }

        const mensagemJson = JSON.stringify({
            nome, email, cpf, dataNascimento, telefone, cep, senhaHash: senhaResultado.hash
        });

        console.log("Mensagem antes da criptografia AES:", mensagemJson);

        const aesKey = CryptoJS.lib.WordArray.random(32); // 256 bits
        const iv = CryptoJS.lib.WordArray.random(16);     // 128 bits

        const encrypted = CryptoJS.AES.encrypt(mensagemJson, aesKey, {
            iv: iv,
            mode: CryptoJS.mode.CBC,
            padding: CryptoJS.pad.Pkcs7
        });

        const encryptedMessageBase64 = encrypted.toString();
        const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);
        const ivBase64 = CryptoJS.enc.Base64.stringify(iv);

        const encryptor = new JSEncrypt();
        encryptor.setPublicKey(publicKeyPem);
        const encryptedAesKey = encryptor.encrypt(aesKeyBase64);

        if (!encryptedAesKey) {
            alert("Erro ao criptografar a chave AES.");
            return;
        }

        const payload = {
            chave_aes_criptografada: encryptedAesKey,
            iv: ivBase64,
            mensagem_criptografada: encryptedMessageBase64
        };
        
        console.log("Payload enviado ao servidor:", payload);


        fetch("/entrada_criar_conta", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        })
        .then(response => response.json())
        .then(resultado => {
            switch (resultado) {
                case 1: window.location.href = "/static/html/login_page.html"; break;
                case 2: alert("Conta já existe com este email ou CPF."); break;
                case 3: alert("Erro ao criar conta. Tente novamente mais tarde."); break;
                default: alert("Erro inesperado.");
            }
        })
        .catch(error => {
            console.error("Erro na requisição:", error);
            alert("Erro ao conectar com o servidor.");
        })
        .finally(() => {
            cadastrarButton.disabled = false;
            cadastrarButton.innerHTML = originalButtonText;
        });
    } else {
        cadastrarButton.disabled = false;
        cadastrarButton.innerHTML = originalButtonText;
    }
}
