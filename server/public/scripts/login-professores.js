document.getElementById('loginForm').addEventListener('submit', function(e) {
    e.preventDefault();
    const email = document.getElementById('email').value;
    fetch('/professores/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ email })
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
