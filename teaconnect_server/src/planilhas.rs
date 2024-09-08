use mysql::*;
use mysql::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Planilha {
    pub id: u32,
    pub id_aluno: u32,
    pub id_usuario: u32,
    pub data: String,
    pub conteudo: String,
}

pub fn create_planilha(conn: &mut PooledConn, id_aluno: u32, id_usuario: u32, data: &str, conteudo: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"INSERT INTO Planilhas (ID_Aluno, ID_Usuario, Data, Conteudo) VALUES (:id_aluno, :id_usuario, :data, :conteudo)",
        params! {
            "id_aluno" => id_aluno,
            "id_usuario" => id_usuario,
            "data" => data,
            "conteudo" => conteudo,
        },
    )?;
    Ok(())
}

pub fn get_planilha(conn: &mut PooledConn, id: u32) -> Result<Planilha, Box<dyn Error>> {
    let result: Option<(u32, u32, u32, String, String)> = conn.exec_first(
        r"SELECT ID, ID_Aluno, ID_Usuario, Data, Conteudo FROM Planilhas WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;

    if let Some((id, id_aluno, id_usuario, data, conteudo)) = result {
        Ok(Planilha { id, id_aluno, id_usuario, data, conteudo })
    } else {
        Err("Planilha não encontrada".into())
    }
}

pub fn update_planilha(conn: &mut PooledConn, id: u32, id_aluno: u32, id_usuario: u32, data: &str, conteudo: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"UPDATE Planilhas SET ID_Aluno = :id_aluno, ID_Usuario = :id_usuario, Data = :data, Conteudo = :conteudo WHERE ID = :id",
        params! {
            "id" => id,
            "id_aluno" => id_aluno,
            "id_usuario" => id_usuario,
            "data" => data,
            "conteudo" => conteudo,
        },
    )?;
    Ok(())
}

pub fn delete_planilha(conn: &mut PooledConn, id: u32) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"DELETE FROM Planilhas WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(())
}
