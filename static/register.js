document.getElementById("login-tab").addEventListener("click", function () {
    window.location.href = "/static/login_page.html";
});

function validarCadastro() {
    let nome = document.getElementById("iname").value;
    let email = document.getElementById("iemail").value;
    let cpf = document.getElementById("icpf").value;
    let dataNascimento = document.getElementById("ibirthdate").value;
    let telefone = document.getElementById("icellphone").value;
    let senha = document.getElementById("password").value;
    let confirmarSenha = document.getElementById("confirmPassword").value;
    let cep = document.getElementById("icep").value;

    if (
        vemail(email) &&
        vcpf(cpf) &&
        vidade(dataNascimento) &&
        vpssw(senha, confirmarSenha) &&
        vtelefone(telefone) &&
        vcep(cep)
    ) {

        fetch('/cfCPFbk', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ cpf: cpf })
        })
        .then(response => response.text())
        .then(data => {
            if (data === "true") {
                fetch("/send_verification", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ email: email, nome: nome }),
                }).then(() => {
                    alert("Cadastro iniciado. Verifique seu e-mail.");
                });
            } else {
                window.alert("CPF inválido! Verificado no back-end.");
            }
        })
        .catch(error => {
            console.error("Erro na verificação do CPF:", error);
            alert("Erro na comunicação com o servidor.");
        });
    } else {
        window.alert("Alguma informação está errada ou inválida. Faça o cadastro de novo.");
    }
}


function vidade(dataNascimento) {
    let dataNasc = new Date(dataNascimento);
    let hoje = new Date();
    let idade = hoje.getFullYear() - dataNasc.getFullYear();
    let mesAtual = hoje.getMonth();
    let diaAtual = hoje.getDate();
    let mesNasc = dataNasc.getMonth();
    let diaNasc = dataNasc.getDate();

    if (mesAtual < mesNasc || (mesAtual === mesNasc && diaAtual < diaNasc)) {
        idade--;
    }

    if (idade >= 18) {
        return true;
    } else {
        window.alert("Você deve ter pelo menos 18 anos para se cadastrar.");
        return false;
    }
}

function vemail(email) {
    const regexEmailPucpr = /^[a-zA-Z0-9._%+-]+@pucpr\.edu\.br$/;
    return regexEmailPucpr.test(email);
}

function vcpf(cpf) {
    cpf = cpf.replace(/\D/g, "");
    if (cpf.length !== 11) {
        window.alert("Erro em CPF.");
        return false;
    }
    return true;
}

// máquina 2 x 1 humano burro
function vpssw(senha1, senha2) {
    if (senha1 !== senha2) return false;
    const regex = /^(?=.*[0-9])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{6,16}$/;
    return regex.test(senha1);
}

function vtelefone(telefone) {
    const regexTelefone = /^\d{10,11}$/;
    return regexTelefone.test(telefone);
}

function vcep(cep) {
    const regexCEP = /^\d{5}-?\d{3}$/;
    return regexCEP.test(cep);
}