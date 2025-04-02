document.getElementById("login-tab").addEventListener("click", function() {
    window.location.href = "/static/login_page.html";
});

function validarCadastro() {
    let nomeCompleto = document.getElementById("iname").value;
    let email = document.getElementById("iemail").value;
    let cpf = document.getElementById("icpf").value;
    let dataNascimento = document.getElementById("ibirthdate").value;
    let telefone = document.getElementById("icellphone").value;
    let senha = document.getElementById("password").value;
    let confirmarSenha = document.getElementById("confirmPassword").value;
    let rua = document.getElementById("istreet").value;
    let numero = document.getElementById("inumero") ? document.getElementById("inumero").value : "";
    let complemento = document.getElementById("icomplemento") ? document.getElementById("icomplemento").value : "";
    let bairro = document.getElementById("ibairro") ? document.getElementById("ibairro").value : "";
    let cidade = document.getElementById("icity").value;
    let estado = document.getElementById("istate").value;
    let cep = document.getElementById("icep").value;

    if (vemail(email) && vcpf(cpf) && vidade(dataNascimento) && vpssw(senha,confirmarSenha )) {
        
    }
    else{
        window.alert("Alguma informacao esta errada ou invalida, faca o cadastro de novo.")
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
function vemail(email) { return true; }

function vcpf(cpf) {
    function vcpf(cpf) {
        cpf = cpf.replace(/\D/g, ""); 

        if (cpf.length !== 11) {
            return false;
        }
    
        return true;
    }
}
function vpssw(senha1, senha2) {
    if (senha1 !== senha2) {
        return false;
    }
    let regex = /^(?=.*[0-9])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{6,16}$/;

    if (!regex.test(senha1)) {
        return false;
    }

    return true;
}

