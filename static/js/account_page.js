let visible = true;

function toggleBalance() {
    const accountBalance = document.getElementById("account-balance");
    const savingsBalance = document.getElementById("savings-balance");
    visible = !visible;

    if (!visible) {
        accountBalance.textContent = "*****";
    } else {
        carregarDadosConta();
    }
}

async function carregarDadosConta() {
    try {
        const response = await fetch("/dados-conta");
        if (!response.ok) throw new Error("Erro ao buscar dados da conta");

        const dados = await response.json();

        // Atualiza saldos
        document.getElementById("account-balance").textContent = dados.saldo_conta;

        // Atualiza transações (mais recente em cima)
        const transactionList = document.getElementById("transaction-list");
        transactionList.innerHTML = "";
        dados.transacoes.slice().reverse().forEach(tx => {
            const li = document.createElement("li");
            li.textContent = tx;
            transactionList.appendChild(li);
        });

    } catch (err) {
        console.error(err);
        alert("Erro ao carregar dados da conta.");
    }
}

function abrirModalDeposito() {
    document.getElementById("modal-deposito").style.display = "block";
    document.getElementById("valor-deposito").value = "";
}

function fecharModalDeposito() {
    document.getElementById("modal-deposito").style.display = "none";
}

async function enviarDeposito(event) {
    event.preventDefault();
    const valor = parseFloat(document.getElementById("valor-deposito").value);
    if (isNaN(valor) || valor <= 0) {
        alert("Digite um valor válido para depósito.");
        return;
    }
    try {
        const resp = await fetch("/depositar", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ valor })
        });
        if (!resp.ok) throw new Error("Erro ao depositar.");
        fecharModalDeposito();
        await carregarDadosConta();
    } catch (err) {
        alert("Erro ao depositar dinheiro.");
    }
}

function abrirModalPagamento() {
    document.getElementById("modal-pagamento").style.display = "block";
    document.getElementById("valor-pagamento").value = "";
}

function fecharModalPagamento() {
    document.getElementById("modal-pagamento").style.display = "none";
}

async function enviarPagamento(event) {
    event.preventDefault();
    const valor = parseFloat(document.getElementById("valor-pagamento").value);
    if (isNaN(valor) || valor <= 0) {
        alert("Digite um valor válido para pagamento.");
        return;
    }
    try {
        const resp = await fetch("/pagar-divida", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ valor })
        });
        if (!resp.ok) throw new Error("Erro ao pagar dívida.");
        fecharModalPagamento();
        await carregarDadosConta();
    } catch (err) {
        alert("Erro ao pagar dívida.");
    }
}

// Fecha modal ao clicar fora do conteúdo
window.onclick = function(event) {
    const modalDep = document.getElementById("modal-deposito");
    const modalPag = document.getElementById("modal-pagamento");
    if (event.target === modalDep) fecharModalDeposito();
    if (event.target === modalPag) fecharModalPagamento();
}

document.addEventListener("DOMContentLoaded", () => {
    carregarDadosConta();
});
