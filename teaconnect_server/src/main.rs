use mysql::*;
use std::error::Error;
use teaconnect_server::usuarios::{create_usuario, delete_usuario, get_usuario, get_usuario_by_nome, update_usuario};
use teaconnect_server::connection::{create_connection, ConnectionType, Connection};
use teaconnect_server::single_thread_server::create_single_thread_server;

fn main() -> Result<(), Box<dyn Error>> {
    // Criar uma conexão com o banco de dados
    let connection_type = ConnectionType::Pool;
    let connection = create_connection(connection_type)?;

    // Obter uma conexão para usar
    let mut conn: PooledConn = match connection {
        Connection::Pool(pool) => pool.get_conn()?,
        Connection::Single(_) => {
            // Se for uma conexão única, então não é possível criar um pool a partir dela.
            return Err("Cannot create a pool from a single connection.".into());
        },
    };

    // Criar um novo usuário
    match create_usuario(&mut conn, "Ana Souza", "ana.souza@example.com", "professor") {
        Ok(_) => println!("Usuário criado com sucesso."),
        Err(err) => println!("Erro ao criar usuário: {}", err),
    }

    // Obter um usuário(aqui ele obtem pelo id). Caso ele tenha sido criado anteriormente, como é autoincrementado no BD, entao não é garantido que o id seja 1.
    match get_usuario(&mut conn, 1) {//aqui o id é sempre 1
        Ok(usuario) => println!("Usuário encontrado: {:?}", usuario),
        Err(err) => println!("Erro ao obter usuário: {}", err),
    }
    
    match get_usuario_by_nome(&mut conn, "Ana Souza") {
        Ok(usuarios) => {
            for usuario in usuarios {
                println!("Usuário encontrado: {:?}", usuario);
            }
        },
        Err(err) => println!("Erro ao obter usuário: {}", err),
    }
    // Atualizar um usuário
    match update_usuario(&mut conn, 1, "Ana Souza", "ana.souza@newemail.com", "professor") {
        Ok(_) => println!("Usuário atualizado com sucesso."),
        Err(err) => println!("Erro ao atualizar usuário: {}", err),
    }

    // Deletar um usuário
    match delete_usuario(&mut conn, 1) {
        Ok(_) => println!("Usuário deletado com sucesso."),
        Err(err) => println!("Erro ao deletar usuário: {}", err),
    }

    match create_single_thread_server() {
        Ok(_) => {
            println!("Servidor iniciado com sucesso.");
           
        },
        Err(err) => println!("Erro ao iniciar servidor: {}", err),
    }

    Ok(())
}

