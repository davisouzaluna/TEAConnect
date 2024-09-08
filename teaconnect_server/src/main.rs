// src/main.rs
use teaconnect_server::connection::{create_connection, ConnectionType, Connection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Criar uma conexão única
    let connection = create_connection(ConnectionType::Single)?;
    match connection {
        Connection::Single(conn) => {
            // Use a conexão única (conn) aqui
            println!("Conectado com sucesso usando uma conexão única!");
        }
        Connection::Pool(_) => {
            println!("Esperava uma conexão única, mas recebeu um pool.");
        }
    }

    // Criar um pool de conexões
    let connection = create_connection(ConnectionType::Pool)?;
    match connection {
        Connection::Pool(pool) => {
            // Use o pool de conexões (pool) aqui
            println!("Conectado com sucesso usando um pool de conexões!");
        }
        Connection::Single(_) => {
            println!("Esperava um pool de conexões, mas recebeu uma conexão única.");
        }
    }

    Ok(())
}
