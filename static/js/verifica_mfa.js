export async function verificarMFA() {
    const codigo = document.getElementById("codigoMFA").value;
    const resultado = document.getElementById("resultado");
  
    const resposta = await fetch('/verifica_mfa', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ codigo })
    });
  
    if (!resposta.ok) {
      resultado.textContent = "❌ Erro ao verificar código!";
      resultado.style.color = "red";
      return;
    }
  
    const valido = await resposta.json();
  
    if (valido === true) {
      resultado.textContent = "✅ Código verificado com sucesso!";
      resultado.style.color = "green";
      setTimeout(() => {
        window.location.href = "/static/html/privada/account_page.html";
      }, 1500);
    } else {
      resultado.textContent = "❌ Código inválido!";
      resultado.style.color = "red";
    }
  }
  