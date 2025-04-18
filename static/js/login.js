document.getElementById("register-tab").addEventListener("click", function () {
    window.location.href = "/static/html/register_page.html";
});

function login(event) {
    event.preventDefault();
  
    const nome = document.getElementById("userId").value;
    const senha = document.getElementById("password").value;
  
    if (nome === "admin@pucpr.edu.br" && senha === "admin") {
      window.location.href = "/static/html/coloca_codigo.html";
    } else {
      alert("Usuário ou senha incorretos");
    }
  
    return false;
}
