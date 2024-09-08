use mysql::*;
use mysql::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CursoOnline {
    pub id: u32,
    pub nome: String,
    pub descricao: String,
    pub link: String,
}

pub fn create_curso_online(conn: &mut PooledConn, nome: &str, descricao: &str, link: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"INSERT INTO CursosOnline (Nome, Descricao, Link) VALUES (:nome, :descricao, :link)",
        params! {
            "nome" => nome,
            "descricao" => descricao,
            "link" => link,
        },
    )?;
    Ok(())
}

pub fn get_curso_online(conn: &mut PooledConn, id: u32) -> Result<CursoOnline, Box<dyn Error>> {
    let result: Option<(u32, String, String, String)> = conn.exec_first(
        r"SELECT ID, Nome, Descricao, Link FROM CursosOnline WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;

    if let Some((id, nome, descricao, link)) = result {
        Ok(CursoOnline { id, nome, descricao, link })
    } else {
        Err("Curso não encontrado".into())
    }
}

pub fn update_curso_online(conn: &mut PooledConn, id: u32, nome: &str, descricao: &str, link: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"UPDATE CursosOnline SET Nome = :nome, Descricao = :descricao, Link = :link WHERE ID = :id",
        params! {
            "id" => id,
            "nome" => nome,
            "descricao" => descricao,
            "link" => link,
        },
    )?;
    Ok(())
}

pub fn delete_curso_online(conn: &mut PooledConn, id: u32) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"DELETE FROM CursosOnline WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(())
}
