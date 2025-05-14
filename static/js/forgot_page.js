document.addEventListener("DOMContentLoaded", () => {
    document.getElementById("send-email").addEventListener("click", async function() {
        const email = document.getElementById("userId").value.trim();
        if (!email) {
            alert("Digite seu e-mail.");
            return;
        }
        try {
            const resp = await fetch("/esqueci_senha", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ email })
            });
            if (resp.ok) {
                alert("Se o e-mail existir, você receberá instruções para recuperar sua conta.");
                
                window.location.href = "/static/html/login_page.html";

            } else if (resp.status === 404) {
                alert("E-mail não encontrado.");
            } else {
                alert("Erro ao enviar e-mail de recuperação.");
            }
        } catch {
            alert("Erro de conexão com o servidor.");
        }
    });
});
