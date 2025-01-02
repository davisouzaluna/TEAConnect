use mysql::*;
use mysql::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaterialDidatico {
    pub id: u32,
    pub titulo: String,
    pub descricao: String,
    pub arquivo: String,
}

pub fn create_material_didatico(conn: &mut PooledConn, titulo: &str, descricao: &str, arquivo: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"INSERT INTO MateriaisDidaticos (Titulo, Descricao, Arquivo) VALUES (:titulo, :descricao, :arquivo)",
        params! {
            "titulo" => titulo,
            "descricao" => descricao,
            "arquivo" => arquivo,
        },
    )?;
    Ok(())
}

pub fn get_material_didatico(conn: &mut PooledConn, id: u32) -> Result<MaterialDidatico, Box<dyn Error>> {
    let result: Option<(u32, String, String, String)> = conn.exec_first(
        r"SELECT ID, Titulo, Descricao, Arquivo FROM MateriaisDidaticos WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;

    if let Some((id, titulo, descricao, arquivo)) = result {
        Ok(MaterialDidatico { id, titulo, descricao, arquivo })
    } else {
        Err("Material não encontrado".into())
    }
}

pub fn update_material_didatico(conn: &mut PooledConn, id: u32, titulo: &str, descricao: &str, arquivo: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"UPDATE MateriaisDidaticos SET Titulo = :titulo, Descricao = :descricao, Arquivo = :arquivo WHERE ID = :id",
        params! {
            "id" => id,
            "titulo" => titulo,
            "descricao" => descricao,
            "arquivo" => arquivo,
        },
    )?;
    Ok(())
}

pub fn delete_material_didatico(conn: &mut PooledConn, id: u32) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"DELETE FROM MateriaisDidaticos WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(())
}
