const cards = [
    {
        label: "Cartão PUC",
        number: "1234 5678 9012 3456",
        limit: "R$ 1.000,00",
        used: "R$ 200,00"
    },
];

function loadCards() {
    const cardsContainer = document.getElementById("cards-container");
    cardsContainer.innerHTML = "";
    cards.forEach(card => {
        const cardDiv = document.createElement("div");
        cardDiv.classList.add("card-summary");
        cardDiv.innerHTML = `
            <div class="card-label">${card.label}</div>
            <div class="card-details">
                <p>Número: ${card.number}</p>
                <p>Limite: ${card.limit}</p>
                <p>Utilizado: ${card.used}</p>
            </div>
        `;
        cardsContainer.appendChild(cardDiv);
    });
}

document.addEventListener("DOMContentLoaded", () => {
    // Função para carregar cartões disponíveis
    async function carregarCartoes() {
        // ...lógica para buscar dados do backend...
    }

    carregarCartoes();
    loadCards();
});
