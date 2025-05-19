document.getElementById('verification-form').addEventListener('submit', async function(event) {
    event.preventDefault();
    
    cpf = document.getElementById('cpf').value;
    codigo = document.getElementById('mfa-code').value; // <- aqui deve ser 'codigo'

    cpf = cpf.replace(/\D/g, '');
    cpf = cpf.toString();
    try {
        const response = await fetch('/verificaEmailAndCriaContaBanco', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ cpf, codigo }),
        });

        if (response.ok) {
            const result = await response.json();
            if (result === true) {
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
