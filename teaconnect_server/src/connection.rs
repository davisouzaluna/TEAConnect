// src/connection.rs
use mysql::{Conn, Pool, OptsBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub enum ConnectionType {
    Single,
    Pool,
}

// Função para carregar o arquivo de configuração
pub fn load_config(file_path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut config = HashMap::new();

    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    
    Ok(config)
}

// Função para criar uma conexão, seja ela única (Single) ou pool (Pool)
pub fn create_connection(connection_type: ConnectionType) -> Result<Connection, Box<dyn Error>> {
    // Carregar as configurações do arquivo config.txt
    let config = load_config("config.txt")?;

    // Obter as variáveis de configuração necessárias
    let host = config.get("DB_HOST").ok_or_else(|| "Faltando DB_HOST".to_string())?;
    let port: u16 = config.get("DB_PORT").ok_or_else(|| "Faltando DB_PORT".to_string())?.parse()?;
    let user = config.get("DB_USER").ok_or_else(|| "Faltando DB_USER".to_string())?;
    let password = config.get("DB_PASSWORD").ok_or_else(|| "Faltando DB_PASSWORD".to_string())?;
    let db_name = config.get("DB_NAME").ok_or_else(|| "Faltando DB_NAME".to_string())?;

    // Criar as opções de conexão
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(host))
        .tcp_port(port)
        .user(Some(user))
        .pass(Some(password))
        .db_name(Some(db_name));

    // Verificar o tipo de conexão e retornar o tipo apropriado
    match connection_type {
        ConnectionType::Single => {
            // Criar uma conexão única
            let conn = Conn::new(opts)?;
            Ok(Connection::Single(conn))
        }
        ConnectionType::Pool => {
            // Criar um pool de conexões
            let pool = Pool::new(opts)?;
            Ok(Connection::Pool(pool))
        }
    }
}

// Enum para encapsular a conexão
pub enum Connection {
    Single(Conn),
    Pool(Pool),
}
