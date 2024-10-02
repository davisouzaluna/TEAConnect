
async function sendDataToServer() {
    const url = 'http://127.0.0.1:7878'; 
    const data = { message: 'Olá, servidor!' }; // Dados que serão enviados

    try {
        
        const response = await fetch(url, {
            method: 'POST', 
            headers: {
                'Content-Type': 'application/json', 
            },
            body: JSON.stringify(data), 
        });

        
        if (response.ok) {
            const responseData = await response.text(); 
            console.log('Resposta do servidor:', responseData); 
        } else {
            console.error('Erro na requisição:', response.statusText);
        }
    } catch (error) {
        console.error('Erro ao enviar dados:', error);
    }
}


sendDataToServer();
