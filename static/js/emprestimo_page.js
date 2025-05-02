const loans = [
    {
        label: "Empréstimo Estudante",
        amount: "R$ 12.500,00",
        interest: "1.5% ao mês"
    },
    {
        label: "Empréstimo Pessoal",
        amount: "R$ 5.000,00",
        interest: "2.0% ao mês"
    }
];

function loadLoans() {
    const loansContainer = document.getElementById("loans-container");
    loansContainer.innerHTML = "";
    loans.forEach((loan, index) => {
        const loanDiv = document.createElement("div");
        loanDiv.classList.add("loan-summary");
        loanDiv.innerHTML = `
            <div class="loan-label">${loan.label}</div>
            <div class="loan-details">
                <p>Valor: ${loan.amount}</p>
                <p>Taxa de Juros: ${loan.interest}</p>
                <button class="loan-button" data-loan-index="${index}">Solicitar</button>
            </div>
        `;
        loansContainer.appendChild(loanDiv);
    });

    document.querySelectorAll(".loan-button").forEach(button => {
        button.addEventListener("click", (event) => {
            const loanIndex = event.target.getAttribute("data-loan-index");
            openLoanForm(loanIndex);
        });
    });
}

function openLoanForm(loanIndex) {
    const loan = loans[loanIndex];
    const formContainer = document.getElementById("loan-form-container");
    formContainer.innerHTML = `
        <h3>Solicitar ${loan.label}</h3>
        <form id="loan-form">
            <label for="name">Nome:</label>
            <input type="text" id="name" name="name" required>
            <label for="amount">Valor:</label>
            <input type="text" id="amount" name="amount" value="${loan.amount}" readonly>
            <label for="interest">Taxa de Juros:</label>
            <input type="text" id="interest" name="interest" value="${loan.interest}" readonly>
            <button type="submit">Enviar Solicitação</button>
        </form>
    `;
    formContainer.style.display = "block";

    document.getElementById("loan-form").addEventListener("submit", (event) => {
        event.preventDefault();
        alert("Solicitação enviada com sucesso!");
        formContainer.style.display = "none";
    });
}

document.addEventListener("DOMContentLoaded", () => {
    // Função para carregar empréstimos disponíveis
    async function carregarEmprestimos() {
        // ...lógica para buscar dados do backend...
    }

    carregarEmprestimos();
    loadLoans();
});
