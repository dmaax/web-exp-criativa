document.getElementById("register-tab").addEventListener("click", function () {
  window.location.href = "/static/html/register_page.html";
});

function login(event) {
  event.preventDefault();

  const email = document.getElementById("userId").value;
  const senha = document.getElementById("password").value;


  const senhaHash = CryptoJS.SHA256(senha).toString(CryptoJS.enc.Hex);

  fetch("/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ email, senha: senhaHash }),
  })
    .then((response) => {
      if (response.ok) {
        return response.json();
      } else {
        throw new Error("Erro na autenticação");
      }
    })
    .then((data) => {
      if (data.success) {
        window.location.href = "/static/html/coloca_codigo.html";
      } else {
        alert("Usuário ou senha incorretos");
      }
    }
    )
    .catch((error) => {
      console.error("Erro:", error);
      alert("Erro na autenticação");
    }
    );

  return false;
}
