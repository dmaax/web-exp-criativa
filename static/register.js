document.getElementById("login-tab").addEventListener("click", function () {
    window.location.href = "/static/login_page.html";
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
        window.alert("Erro 1 em CPF.");
        return false;
    }
    return true;
}

function verificacpfbkend(c2) {
    return fetch('/cfCPFbk', {
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
        console.error('Erro ao verificar CPF:', error);
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
        window.alert("Você deve ter pelo menos 18 anos para se cadastrar.");
        return false;
    }
}

function resultadosenha(s1, s2) {
    if (s1 !== s2) {
        return false;
    }

    const regex = /^(?=.*[0-9])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{6,16}$/;

    if (regex.test(s1)) {
        const senha = document.getElementById("password").value;
        const hash = CryptoJS.SHA256(senha).toString(CryptoJS.enc.Hex);
        console.log("Hash da senha:", hash);
        return true;
    } else {
        return false;
    }
}


function resultadotelefone(telefone) {
    const regexTelefone = /^\d{10,11}$/;
    if (!regexTelefone.test(telefone)) {
        alert("Telefone inválido. Deve conter 10 ou 11 dígitos numéricos.");
        return false;
    }
    return true;
}

function resultadocep(cp) {
    const regexCEP = /^\d{5}-?\d{3}$/;
    if (!regexCEP.test(cp)) {
        alert("CEP inválido. Formato esperado: 00000000.");
        return false;
    }
    return true;
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
    let resultadosenhaok = resultadosenha(senha, confirmarSenha);
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


    if (resultadocpfbasico &&resultadoemail && resultadocpfbkend && resultadoidade &&resultadosenhaok && resultadotelefoneok && resultadocepok) {
        window.alert("Email enviado para confirmar conta");
    
        fetch("/send_verification", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ email: email, nome: nome }),
        })
    }
}




  