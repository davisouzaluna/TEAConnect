use mysql::{Conn, Pool, OptsBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub enum ConnectionType {
    Single,
    Pool,
}

// Função para carregar o arquivo de configuração. Se atente que o arquivo de configuração deve estar no mesmo diretório que o executável ou no diretório raiz do projeto.
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

// Função para obter as opções de configuração(opts) do banco de dados
pub fn get_opts() -> Result<OptsBuilder, Box<dyn Error>> {
    let config = load_config("config.txt")?;

    let host = config.get("DB_HOST").ok_or_else(|| "Faltando DB_HOST".to_string())?;
    let port: u16 = config.get("DB_PORT").ok_or_else(|| "Faltando DB_PORT".to_string())?.parse()?;
    let user = config.get("DB_USER").ok_or_else(|| "Faltando DB_USER".to_string())?;
    let password = config.get("DB_PASSWORD").ok_or_else(|| "Faltando DB_PASSWORD".to_string())?;
    let db_name = config.get("DB_NAME").ok_or_else(|| "Faltando DB_NAME".to_string())?;

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(host))
        .tcp_port(port)
        .user(Some(user))
        .pass(Some(password))
        .db_name(Some(db_name));

    Ok(opts)
}

// Função para criar uma conexão, seja ela única (Single) ou pool (Pool)
pub fn create_connection(connection_type: ConnectionType) -> Result<Connection, Box<dyn Error>> {
    let opts = get_opts()?;

    match connection_type {
        ConnectionType::Single => {
            let conn = Conn::new(opts)?;
            Ok(Connection::Single(conn))
        }
        ConnectionType::Pool => {
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
