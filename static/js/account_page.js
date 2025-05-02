const balances = {
    account: "R$ 100,56",
    savings: "R$ 50,00"
};

const transactions = [
    "Compra no Mercado - R$ 30,00",
    "Depósito - R$ 100,00",
    "Pagamento de Conta - R$ 50,00"
];

let visible = true;

function toggleBalance() {
    const accountBalance = document.getElementById("account-balance");
    const savingsBalance = document.getElementById("savings-balance");
    visible = !visible;
    accountBalance.textContent = visible ? balances.account : "*****";
    savingsBalance.textContent = visible ? balances.savings : "*****";
}

function loadTransactions() {
    const transactionList = document.getElementById("transaction-list");
    transactionList.innerHTML = "";
    transactions.forEach(transaction => {
        const li = document.createElement("li");
        li.textContent = transaction;
        transactionList.appendChild(li);
    });
}

function loadBalances() {
    document.getElementById("account-balance").textContent = balances.account;
    document.getElementById("savings-balance").textContent = balances.savings;
}

document.addEventListener("DOMContentLoaded", () => {
    loadBalances();
    loadTransactions();

    // Função para carregar dados do backend
    async function carregarDadosConta() {
        // ...lógica para buscar dados do backend...
    }

    carregarDadosConta();
});
