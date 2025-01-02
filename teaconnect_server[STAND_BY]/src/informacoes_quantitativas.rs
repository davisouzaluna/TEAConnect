use mysql::*;
use mysql::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct InformacaoQuantitativa {
    pub id: u32,
    pub id_aluno: u32,
    pub id_usuario: u32,
    pub id_planilha: u32,
    pub tipo_informacao: String,
    pub valor: f32,
    pub data: String,
}

pub fn create_informacao_quantitativa(conn: &mut PooledConn, id_aluno: u32, id_usuario: u32, id_planilha: u32, tipo_informacao: &str, valor: f32, data: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"INSERT INTO InformacoesQuantitativas (ID_Aluno, ID_Usuario, ID_Planilha, TipoInformacao, Valor, Data) VALUES (:id_aluno, :id_usuario, :id_planilha, :tipo_informacao, :valor, :data)",
        params! {
            "id_aluno" => id_aluno,
            "id_usuario" => id_usuario,
            "id_planilha" => id_planilha,
            "tipo_informacao" => tipo_informacao,
            "valor" => valor,
            "data" => data,
        },
    )?;
    Ok(())
}

pub fn get_informacao_quantitativa(conn: &mut PooledConn, id: u32) -> Result<InformacaoQuantitativa, Box<dyn Error>> {
    let result: Option<(u32, u32, u32, u32, String, f32, String)> = conn.exec_first(
        r"SELECT ID, ID_Aluno, ID_Usuario, ID_Planilha, TipoInformacao, Valor, Data FROM InformacoesQuantitativas WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;

    if let Some((id, id_aluno, id_usuario, id_planilha, tipo_informacao, valor, data)) = result {
        Ok(InformacaoQuantitativa { id, id_aluno, id_usuario, id_planilha, tipo_informacao, valor, data })
    } else {
        Err("Informação não encontrada".into())
    }
}

pub fn update_informacao_quantitativa(conn: &mut PooledConn, id: u32, id_aluno: u32, id_usuario: u32, id_planilha: u32, tipo_informacao: &str, valor: f32, data: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"UPDATE InformacoesQuantitativas SET ID_Aluno = :id_aluno, ID_Usuario = :id_usuario, ID_Planilha = :id_planilha, TipoInformacao = :tipo_informacao, Valor = :valor, Data = :data WHERE ID = :id",
        params! {
            "id" => id,
            "id_aluno" => id_aluno,
            "id_usuario" => id_usuario,
            "id_planilha" => id_planilha,
            "tipo_informacao" => tipo_informacao,
            "valor" => valor,
            "data" => data,
        },
    )?;
    Ok(())
}

pub fn delete_informacao_quantitativa(conn: &mut PooledConn, id: u32) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        r"DELETE FROM InformacoesQuantitativas WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(())
}
