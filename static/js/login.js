document.getElementById("register-tab").addEventListener("click", function () {
  window.location.href = "/static/html/register_page.html";
});

function marcarCampoInvalido(idCampo, invalido) {
  const campo = document.getElementById(idCampo);
  if (invalido) {
    campo.classList.add("campo-invalido");
  } else {
    campo.classList.remove("campo-invalido");
  }
}


async function getPublicKey() {
    try {
        const response = await fetch("/pega-chave");
        const data = await response.json();
        console.log("Chave pública recebida:", data);
        return data.chavepb;
    } catch (error) {
        console.error("Erro ao obter chave pública:", error);
        return null;
    }
}

async function login(event) {
  event.preventDefault();

  const loginButton = document.querySelector(".login-button");
  const originalButtonText = loginButton.innerHTML;

  loginButton.disabled = true;
  loginButton.innerHTML = `<span class="spinner"></span> Processando...`;

  const email = document.getElementById("userId").value;
  const senha = document.getElementById("password").value;

  let emailValido = email.trim() !== "";
  let senhaValida = senha.trim() !== "";

  marcarCampoInvalido("userId", !emailValido);
  marcarCampoInvalido("password", !senhaValida);

  if (!emailValido || !senhaValida) {
    loginButton.disabled = false;
    loginButton.innerHTML = originalButtonText;
    return;
  }

  const senhaHash = CryptoJS.SHA256(senha).toString(CryptoJS.enc.Hex);
  console.log("Hash da senha:", senhaHash);

  const publicKeyPem = await getPublicKey();
  if (!publicKeyPem) {
    alert("Erro ao obter a chave pública do servidor.");
    return;
  }

  const mensagemJson = JSON.stringify({
    email: email,
    senha: senhaHash,
  });

  console.log("Mensagem antes da criptografia AES:", mensagemJson);

  const aesKey = CryptoJS.lib.WordArray.random(32); // 256 bits
  console.log("AES Key (Base64):", CryptoJS.enc.Base64.stringify(aesKey));
  const iv = CryptoJS.lib.WordArray.random(16);     // 128 bits

  const encrypted = CryptoJS.AES.encrypt(mensagemJson, aesKey, {
    iv: iv,
    mode: CryptoJS.mode.CBC,
    padding: CryptoJS.pad.Pkcs7
  });

  const encryptedMessageBase64 = encrypted.toString();
  const aesKeyBase64 = CryptoJS.enc.Base64.stringify(aesKey);
  const ivBase64 = CryptoJS.enc.Base64.stringify(iv);

  console.log("Mensagem criptografada (Base64):", encryptedMessageBase64);
  console.log("IV (Base64):", ivBase64);

  const encryptor = new JSEncrypt();
  encryptor.setPublicKey(publicKeyPem);
  const encryptedAesKey = encryptor.encrypt(aesKeyBase64);

  if (!encryptedAesKey) {
    alert("Erro ao criptografar a chave AES.");
    return;
  }

  const payload = {
    chave_aes_criptografada: encryptedAesKey,
    iv: ivBase64,
    mensagem_criptografada: encryptedMessageBase64
  };
  console.log("Payload enviado ao servidor:", payload);

  try {
    const response = await fetch("/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });
    console.log("Corpo enviado no fetch:", JSON.stringify(payload));
    console.log("Status HTTP da resposta:", response.status);

    if (response.ok) {
      const success = await response.json();
      console.log("Resposta do servidor (JSON):", success);
      if (success) {
        window.location.href = "/static/html/coloca_codigo.html";
      } else {
        alert("Usuário ou senha incorretos");
      }
    } else {
      const errorText = await response.text();
      console.error("Erro na resposta do servidor:", errorText);
      throw new Error("Erro na autenticação");
    }
  } catch (error) {
    console.error("Erro:", error);
    alert("Erro na autenticação");
  } finally {
    loginButton.disabled = false;
    loginButton.innerHTML = originalButtonText;
  }
}
