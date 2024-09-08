use mysql::*;
use mysql::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Usuario {
    pub id: u32,
    pub nome: String,
    pub email: String,
    pub tipo: String,
}

pub fn create_usuario(conn: &mut PooledConn, nome: &str, email: &str, tipo: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"INSERT INTO Usuarios (Nome, Email, Tipo) VALUES (:nome, :email, :tipo)",
        params! {
            "nome" => nome,
            "email" => email,
            "tipo" => tipo,
        },
    )?;
    Ok(())
}

pub fn get_usuario(conn: &mut PooledConn, id: u32) -> Result<Usuario, Box<dyn Error>> {
    let result: Option<(u32, String, String, String)> = conn.exec_first(
        r"SELECT ID, Nome, Email, Tipo FROM Usuarios WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;

    if let Some((id, nome, email, tipo)) = result {
        Ok(Usuario { id, nome, email, tipo })
    } else {
        Err("Usuario não encontrado".into())
    }
}

pub fn update_usuario(conn: &mut PooledConn, id: u32, nome: &str, email: &str, tipo: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"UPDATE Usuarios SET Nome = :nome, Email = :email, Tipo = :tipo WHERE ID = :id",
        params! {
            "id" => id,
            "nome" => nome,
            "email" => email,
            "tipo" => tipo,
        },
    )?;
    Ok(())
}
pub fn get_usuario_by_nome(conn: &mut PooledConn, nome: &str) -> Result<Vec<Usuario>, Box<dyn Error>> {
    let results: Vec<(u32, String, String, String)> = conn.exec(
        r"SELECT ID, Nome, Email, Tipo FROM Usuarios WHERE Nome = :nome",
        params! {
            "nome" => nome,
        },
    )?;

    let usuarios: Vec<Usuario> = results.into_iter()
        .map(|(id, nome, email, tipo)| Usuario { id, nome, email, tipo })
        .collect();

    Ok(usuarios)
}
pub fn delete_usuario(conn: &mut PooledConn, id: u32) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"DELETE FROM Usuarios WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(())
}
