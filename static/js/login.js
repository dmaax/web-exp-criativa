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

  try {
    const response = await fetch("/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, senha: senhaHash }),
    });

    if (response.ok) {
      const success = await response.json();
      if (success) {
        window.location.href = "/static/html/coloca_codigo.html";
      } else {
        alert("Usuário ou senha incorretos");
      }
    } else {
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
