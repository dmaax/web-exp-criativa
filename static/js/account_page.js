let visible = true;
let currentAesKey = null; // Armazenar a chave AES atual

async function decryptResponse(encryptedData, iv, aesKey) {
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

    return JSON.parse(decrypted.toString(CryptoJS.enc.Utf8));
}

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
        // Gerar chave AES e IV para a requisição
        const aesKey = CryptoJS.lib.WordArray.random(32);
        currentAesKey = aesKey;
        const iv = CryptoJS.lib.WordArray.random(16);

        // Pegar chave pública e criptografar a chave AES
        const publicKeyPem = await (await fetch("/pega-chave")).json();
        const encryptor = new JSEncrypt();
        encryptor.setPublicKey(publicKeyPem.chavepb);
        const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);
        const encryptedAesKey = encryptor.encrypt(aesKeyBase64);

        // Preparar payload
        const payload = {
            chave_aes_criptografada: encryptedAesKey,
            iv: CryptoJS.enc.Base64.stringify(iv),
            mensagem_criptografada: "" // Vazio pois é um GET
        };

        const response = await fetch("/dados-conta", {
            method: "POST",  // Mudando para POST para enviar o payload
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });

        if (response.status === 401) {
            window.location.href = "/static/html/login_page.html";
            return;
        }
        if (!response.ok) throw new Error("Erro ao buscar dados da conta");

        const encryptedData = await response.json();
        const dados = await decryptResponse(encryptedData.encrypted_data, encryptedData.iv, currentAesKey);

        // Atualiza saldos
        if (visible) {
            document.getElementById("account-balance").textContent = dados.saldo_conta;
        }

        // Atualiza transações
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
        // Gerar chave AES e IV
        const aesKey = CryptoJS.lib.WordArray.random(32);
        currentAesKey = aesKey; // Salvar a chave AES
        const iv = CryptoJS.lib.WordArray.random(16);

        // Criptografar mensagem
        const mensagemJson = JSON.stringify({ valor });
        const encrypted = CryptoJS.AES.encrypt(mensagemJson, aesKey, {
            iv: iv,
            mode: CryptoJS.mode.CBC,
            padding: CryptoJS.pad.Pkcs7
        });

        const encryptedMessageBase64 = encrypted.toString();
        const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);
        const ivBase64 = CryptoJS.enc.Base64.stringify(iv);

        // Criptografar chave AES com RSA
        const publicKeyPem = await (await fetch("/pega-chave")).json();
        const encryptor = new JSEncrypt();
        encryptor.setPublicKey(publicKeyPem.chavepb);
        const encryptedAesKey = encryptor.encrypt(aesKeyBase64);

        const payload = {
            chave_aes_criptografada: encryptedAesKey,
            iv: ivBase64,
            mensagem_criptografada: encryptedMessageBase64
        };

        const resp = await fetch("/depositar", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });
        if (!resp.ok) throw new Error("Erro ao depositar.");
        
        const encryptedData = await resp.json();
        await decryptResponse(encryptedData.encrypted_data, encryptedData.iv, currentAesKey);
        
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
        // Gerar chave AES e IV
        const aesKey = CryptoJS.lib.WordArray.random(32);
        currentAesKey = aesKey; // Salvar a chave AES
        const iv = CryptoJS.lib.WordArray.random(16);

        // Criptografar mensagem
        const mensagemJson = JSON.stringify({ valor });
        const encrypted = CryptoJS.AES.encrypt(mensagemJson, aesKey, {
            iv: iv,
            mode: CryptoJS.mode.CBC,
            padding: CryptoJS.pad.Pkcs7
        });

        const encryptedMessageBase64 = encrypted.toString();
        const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);
        const ivBase64 = CryptoJS.enc.Base64.stringify(iv);

        // Criptografar chave AES com RSA
        const publicKeyPem = await (await fetch("/pega-chave")).json();
        const encryptor = new JSEncrypt();
        encryptor.setPublicKey(publicKeyPem.chavepb);
        const encryptedAesKey = encryptor.encrypt(aesKeyBase64);

        const payload = {
            chave_aes_criptografada: encryptedAesKey,
            iv: ivBase64,
            mensagem_criptografada: encryptedMessageBase64
        };

        const resp = await fetch("/pagar-divida", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload)
        });
        if (!resp.ok) throw new Error("Erro ao pagar dívida.");
        
        const encryptedData = await resp.json();
        await decryptResponse(encryptedData.encrypted_data, encryptedData.iv, currentAesKey);
        
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
