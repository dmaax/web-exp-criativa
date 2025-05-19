async function carregarCartao() {
    try {
        const resp = await fetch("/cartoes");
        if (resp.status === 401) {
            window.location.href = "/static/html/login_page.html";
            return;
        }
        if (!resp.ok) throw new Error("Erro ao buscar cartão");
        const card = await resp.json();
        const cardInfo = document.getElementById("card-info");
        if (!card || !card.id) {
            cardInfo.innerHTML = "<p>Nenhum cartão encontrado.</p>";
            return;
        }
        cardInfo.innerHTML = `
            <div class="cartao-box">
                <div class="cartao-numero"><span>Número:</span> ${card.numero}</div>
                <div class="cartao-dados">
                    <span>Validade:</span> ${card.data_cartao} &nbsp; 
                    <span>Código:</span> ${card.codigo_cartao}
                </div>
                <div class="cartao-limite">
                    <span>Limite:</span> ${card.limite} &nbsp; 
                    <span>Utilizado:</span> ${card.usado}
                </div>
            </div>
        `;
        // Salva o id do cartão para uso no submit
        window.cartao_id = card.id;
    } catch (err) {
        document.getElementById("card-info").innerHTML = "<p>Erro ao carregar cartão.</p>";
    }
}

document.addEventListener("DOMContentLoaded", () => {
    carregarCartao();

    document.getElementById("purchase-form").addEventListener("submit", async (event) => {
        event.preventDefault();
        const nome = document.getElementById("purchase-name").value;
        const valor = parseFloat(document.getElementById("purchase-value").value);
        const cartao_id = window.cartao_id;
        if (!nome || isNaN(valor) || valor <= 0 || !cartao_id) {
            alert("Preencha os dados corretamente.");
            return;
        }
        try {
            const resp = await fetch("/compra", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ cartao_id, nome_compra: nome, valor })
            });
            if (resp.status === 401) {
                window.location.href = "/static/html/login_page.html";
                return;
            }
            if (!resp.ok) {
                alert("Compra não autorizada (limite insuficiente ou erro).");
                return;
            }
            document.getElementById("purchase-success").style.display = "block";
            setTimeout(() => {
                document.getElementById("purchase-success").style.display = "none";
            }, 2000);
            document.getElementById("purchase-form").reset();
            carregarCartao();
        } catch {
            alert("Erro ao registrar compra.");
        }
    });
});
