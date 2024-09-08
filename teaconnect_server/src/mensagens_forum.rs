use mysql::*;
use mysql::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MensagemForum {
    pub id: u32,
    pub id_usuario: u32,
    pub conteudo: String,
    pub data: String,
}

pub fn create_mensagem_forum(conn: &mut PooledConn, id_usuario: u32, conteudo: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"INSERT INTO MensagensForum (ID_Usuario, Conteudo) VALUES (:id_usuario, :conteudo)",
        params! {
            "id_usuario" => id_usuario,
            "conteudo" => conteudo,
        },
    )?;
    Ok(())
}

pub fn get_mensagem_forum(conn: &mut PooledConn, id: u32) -> Result<MensagemForum, Box<dyn Error>> {
    let result: Option<(u32, u32, String, String)> = conn.exec_first(
        r"SELECT ID, ID_Usuario, Conteudo, Data FROM MensagensForum WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;

    if let Some((id, id_usuario, conteudo, data)) = result {
        Ok(MensagemForum { id, id_usuario, conteudo, data })
    } else {
        Err("Mensagem não encontrada".into())
    }
}

pub fn update_mensagem_forum(conn: &mut PooledConn, id: u32, conteudo: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"UPDATE MensagensForum SET Conteudo = :conteudo WHERE ID = :id",
        params! {
            "id" => id,
            "conteudo" => conteudo,
        },
    )?;
    Ok(())
}

pub fn delete_mensagem_forum(conn: &mut PooledConn, id: u32) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"DELETE FROM MensagensForum WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(())
}
