use std::net::TcpListener;
use std::io::{Read,Write};

pub fn create_single_thread_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor escutando na porta 7878.");

    for stream in listener.incoming() {
        let mut stream = stream?;
        handle_connection(&mut stream);
    }

    Ok(())
}

fn handle_connection(stream: &mut std::net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    let post_request = b"POST / HTTP/1.1\r\n";

    if buffer.starts_with(post_request) {
        let contents = extract_json_from_request(&request);
        let response = format!("HTTP/1.1 200 OK\r\n\r\nRecebi os dados: {}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn extract_json_from_request(request: &str) -> &str {
    let parts: Vec<&str> = request.split("\r\n\r\n").collect();
    if parts.len() > 1 {
        return parts[1]; //Retorna o conteúdo do JSON
    }
    ""
}