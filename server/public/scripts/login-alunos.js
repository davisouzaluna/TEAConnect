document.getElementById('loginForm').addEventListener('submit', function(e) {
    e.preventDefault();
    const nome = document.getElementById('nome').value;
    fetch('/alunos/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ nome })
    })
    .then(response => response.json())
    .then(data => {
        if (data.error) {
            document.getElementById('userInfo').textContent = data.error;
        } else {
            document.getElementById('userInfo').textContent = JSON.stringify(data);
        }
    });
});
