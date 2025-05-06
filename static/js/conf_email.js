document.getElementById('verification-form').addEventListener('submit', async function(event) {
    event.preventDefault();
    const cpf = document.getElementById('cpf').value;
    const mfaCode = document.getElementById('mfa-code').value;

    try {
        const response = await fetch('/verificaEmailAndCriaContaBanco', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ cpf, mfaCode }),
        });

        if (response.ok) {
            const result = await response.json();
            if (result.success) {
                window.location.href = '/static/html/login_page.html';
            } else {
                alert('Erro na verificação. Por favor, tente novamente.');
            }
        } else {
            alert('Erro no servidor. Por favor, tente novamente mais tarde.');
        }
    } catch (error) {
        alert('Erro de conexão. Por favor, tente novamente.');
    }
});
