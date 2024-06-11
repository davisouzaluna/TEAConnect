document.getElementById('cadastroForm').addEventListener('submit', function(e) {
    e.preventDefault();
    const nome = document.getElementById('nome').value;
    const email = document.getElementById('email').value;
    const tipo = document.getElementById('tipo').value;
    
    fetch('/cadastro', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ nome, email, tipo })
    })
    .then(response => response.json())
    .then(data => {
        alert(data.message);
        // Limpar o formulário após o cadastro
        document.getElementById('cadastroForm').reset();
    })
    .catch(error => {
        console.error('Erro:', error);
    });
});
