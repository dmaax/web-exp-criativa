let currentCardAesKey = null;

async function decryptCardResponse(encryptedData, iv, aesKey) {
    try {
        console.log("Dados criptografados recebidos:", { encryptedData, iv });
        const ivParsed = CryptoJS.enc.Base64.parse(iv);
        const decrypted = CryptoJS.AES.decrypt(
            encryptedData,
            aesKey,
            {
                iv: ivParsed,
                mode: CryptoJS.mode.CBC,
                padding: CryptoJS.pad.Pkcs7
            }
        );
        
        const decryptedStr = decrypted.toString(CryptoJS.enc.Utf8);
        console.log("Dados descriptografados:", decryptedStr);
        
        return JSON.parse(decryptedStr);
    } catch (err) {
        console.error("Erro na descriptografia:", err);
        throw err;
    }
}

async function carregarCartao() {
    try {
        // Gerar e salvar chave AES
        const aesKey = CryptoJS.lib.WordArray.random(32);
        currentCardAesKey = aesKey;
        const iv = CryptoJS.lib.WordArray.random(16);

        console.log("Obtendo chave pública...");
        const publicKeyPem = await (await fetch("/pega-chave")).json();
        const encryptor = new JSEncrypt();
        encryptor.setPublicKey(publicKeyPem.chavepb);

        const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);
        const encryptedAesKey = encryptor.encrypt(aesKeyBase64);

        const payload = {
            chave_aes_criptografada: encryptedAesKey,
            iv: CryptoJS.enc.Base64.stringify(iv),
            mensagem_criptografada: ""
        };

        console.log("Enviando requisição");
        const resp = await fetch("/cartoes", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });
        if (resp.status === 401) {
            window.location.href = "/static/html/login_page.html";
            return;
        }
        if (!resp.ok) throw new Error("Erro ao buscar cartão");
        const encryptedData = await resp.json();
        console.log("Resposta criptografada recebida:", encryptedData);

        const card = await decryptCardResponse(
            encryptedData.encrypted_data,
            encryptedData.iv,
            currentCardAesKey
        );


        if (!card || !card.id) {
            throw new Error("Dados do cartão inválidos");
        }

        // Atualizar UI com validações
        const cardInfo = document.getElementById("card-info");
        cardInfo.innerHTML = `
            <div class="cartao-box">
                <div class="cartao-numero">
                    <span>Número:</span> ${card.numero || 'N/A'}
                </div>
                <div class="cartao-dados">
                    <span>Validade:</span> ${card.data_cartao || 'N/A'} &nbsp; 
                    <span>Código:</span> ${card.codigo_cartao || 'N/A'}
                </div>
                <div class="cartao-limite">
                    <span>Limite:</span> ${card.limite || 'R$ 0,00'} &nbsp; 
                    <span>Utilizado:</span> ${card.usado || 'R$ 0,00'}
                </div>
            </div>
        `;

        window.cartao_id = card.id;
        console.log("Cartão carregado com sucesso");

    } catch (err) {
        console.error("Erro detalhado:", err);
        document.getElementById("card-info").innerHTML = `
            <p>Erro ao carregar cartão: ${err.message}</p>
        `;
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
            const aesKey = CryptoJS.lib.WordArray.random(32);
            currentCardAesKey = aesKey;
            const iv = CryptoJS.lib.WordArray.random(16);

            const mensagemJson = JSON.stringify({ 
                cartao_id, 
                nome_compra: nome, 
                valor 
            });

            const encrypted = CryptoJS.AES.encrypt(mensagemJson, aesKey, {
                iv: iv,
                mode: CryptoJS.mode.CBC,
                padding: CryptoJS.pad.Pkcs7
            });

            const publicKeyPem = await (await fetch("/pega-chave")).json();
            const encryptor = new JSEncrypt();
            encryptor.setPublicKey(publicKeyPem.chavepb);
            const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);

            const payload = {
                chave_aes_criptografada: encryptor.encrypt(aesKeyBase64),
                iv: CryptoJS.enc.Base64.stringify(iv),
                mensagem_criptografada: encrypted.toString()
            };

            const resp = await fetch("/compra", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(payload)
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
        } catch (err) {
            console.error(err);
            alert("Erro ao registrar compra.");
        }
    });
});
