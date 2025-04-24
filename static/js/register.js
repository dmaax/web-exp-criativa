document.getElementById("login-tab").addEventListener("click", function () {
    window.location.href = "/static/html/login_page.html";
});

function verificaemail(e) {
    const regexEmailPucpr = /^[a-zA-Z0-9._%+-]+@pucpr\.edu\.br$/;
    if (regexEmailPucpr.test(e)) {
        return true;
    } else {
        return false;
    }
}

function verificacpfbasico(c) {
    if (c.length !== 11) {
        return false;
    }
    return true;
}

function verificacpfbkend(c2) {
    return fetch('/verifica_cpf', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ cpf: c2 })
    })
    .then(response => {
        if (response.ok) {
            return response.json();
        } else {
            throw new Error('Erro na requisição');
        }
    })
    .then(data => {
        return data.valido;
    })
    .catch(error => {
        return false;
    });
}

function verificaidade(i) {
    let dataNasc = new Date(i);
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
        return false;
    }
}

function resultadosenha(s1, s2) {
    if (s1 !== s2) {
        return { valido: false, hash: null };
    }

    const regex = /^(?=.*[0-9])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{6,16}$/;

    if (regex.test(s1)) {
        const hash = CryptoJS.SHA256(s1).toString(CryptoJS.enc.Hex);
        return { valido: true, hash: hash };
    } else {
        return { valido: false, hash: null };
    }
}



function resultadotelefone(telefone) {
    const regexTelefone = /^\d{10,11}$/;
    if (!regexTelefone.test(telefone)) {
        return false;
    }
    return true;
}

function resultadocep(cp) {
    const regexCEP = /^\d{5}-?\d{3}$/;
    if (!regexCEP.test(cp)) {
        return false;
    }
    return true;
}

function marcarCampoInvalido(idCampo, invalido) {
    const campo = document.getElementById(idCampo);
    if (invalido) {
        campo.classList.add("campo-invalido");
    } else {
        campo.classList.remove("campo-invalido");
    }
}



async function validarCadastro() {
    let nome = document.getElementById("iname").value;
    let email = document.getElementById("iemail").value;
    let cpf = document.getElementById("icpf").value.toString();
    let dataNascimento = document.getElementById("ibirthdate").value;
    let telefone = document.getElementById("icellphone").value;
    let senha = document.getElementById("password").value;
    let confirmarSenha = document.getElementById("confirmPassword").value;
    let cep = document.getElementById("icep").value;


    cpf = cpf.replace(/\D/g, '');
    telefone = telefone.replace(/\D/g, '');
    cep = cep.replace(/\D/g, '');

    let resultadoemail = verificaemail(email.toString());
    let resultadocpfbasico = verificacpfbasico(cpf.toString());
    let resultadocpfbkend = false;


    if (resultadocpfbasico) {
        try {
            resultadocpfbkend = await verificacpfbkend(cpf.toString());
        } catch (e) {
            console.error("Erro ao verificar CPF no backend:", e);
            resultadocpfbkend = false;
        }
    }

    let resultadoidade = verificaidade(dataNascimento);
    let senhaResultado = resultadosenha(senha, confirmarSenha);
    let resultadosenhaok = senhaResultado.valido;
    let hashFinal = senhaResultado.hash;

    let resultadotelefoneok = resultadotelefone(telefone);
    let resultadocepok = resultadocep(cep);

    console.log("Validação individual:");
    console.log("resultadocpfbasico:", resultadocpfbasico);
    console.log("resultadoemail:", resultadoemail);
    console.log("resultadocpfbkend:", resultadocpfbkend);
    console.log("resultadoidade:", resultadoidade);
    console.log("resultadosenhaok:", resultadosenhaok);
    console.log("resultadotelefoneok:", resultadotelefoneok);
    console.log("resultadocepok:", resultadocepok);

    marcarCampoInvalido("iemail", !resultadoemail);
    marcarCampoInvalido("icpf", !(resultadocpfbasico && resultadocpfbkend));
    marcarCampoInvalido("ibirthdate", !resultadoidade);
    marcarCampoInvalido("password", !resultadosenhaok);
    marcarCampoInvalido("confirmPassword", !resultadosenhaok);
    marcarCampoInvalido("icellphone", !resultadotelefoneok);
    marcarCampoInvalido("icep", !resultadocepok);


    if (
        resultadocpfbasico &&
        resultadoemail &&
        resultadocpfbkend &&
        resultadoidade &&
        resultadosenhaok &&
        resultadotelefoneok &&
        resultadocepok
    ) {
        fetch("/entrada_criar_conta", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                email: email,
                nome: nome,
                cpf: cpf,
                dataNascimento: dataNascimento,
                telefone: telefone,
                cep: cep,
                senha: hashFinal
            }),
        })
        .then(response => response.json())
        .then(resultado => {
            switch (resultado) {
                case 1:
                    setTimeout(() => {
                        window.location.href = "/static/html/login_page.html";
                    }, 3000);
                    break;
                case 2:
                    alert("Conta já existe com este email ou CPF.");
                    break;
                case 3:
                    alert("Erro ao criar conta. Tente novamente mais tarde.");
                    break;
                default:
                    alert("Erro inesperado.");
            }
        })
        .catch(error => {
            console.error("Erro na requisição:", error);
            alert("Erro ao conectar com o servidor.");
        });
    }
}





