use mysql::*;
use mysql::prelude::*;
use std::error::Error;

// Função para inserir um aluno
pub fn create_aluno(pool: &Pool, nome: &str, data_nascimento: &str, diagnostico_tea: &str) -> Result<(), Box<dyn Error>> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        r"INSERT INTO Alunos (Nome, DataNascimento, DiagnosticoTEA) VALUES (:nome, :data_nascimento, :diagnostico_tea)",
        params! {
            "nome" => nome,
            "data_nascimento" => data_nascimento,
            "diagnostico_tea" => diagnostico_tea,
        },
    )?;
    Ok(())
}

// Função para ler todos os alunos
pub fn read_alunos(pool: &Pool) -> Result<Vec<(u32, String, String, String)>, Box<dyn Error>> {
    let mut conn = pool.get_conn()?;
    let result = conn.query_map(
        r"SELECT ID, Nome, DataNascimento, DiagnosticoTEA FROM Alunos",
        |(id, nome, data_nascimento, diagnostico_tea)| {
            (id, nome, data_nascimento, diagnostico_tea)
        },
    )?;
    Ok(result)
}

// Função para atualizar um aluno
pub fn update_aluno(pool: &Pool, id: u32, nome: &str, data_nascimento: &str, diagnostico_tea: &str) -> Result<(), Box<dyn Error>> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        r"UPDATE Alunos SET Nome = :nome, DataNascimento = :data_nascimento, DiagnosticoTEA = :diagnostico_tea WHERE ID = :id",
        params! {
            "id" => id,
            "nome" => nome,
            "data_nascimento" => data_nascimento,
            "diagnostico_tea" => diagnostico_tea,
        },
    )?;
    Ok(())
}

// Função para deletar um aluno
pub fn delete_aluno(pool: &Pool, id: u32) -> Result<(), Box<dyn Error>> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        r"DELETE FROM Alunos WHERE ID = :id",
        params! {
            "id" => id,
        },
    )?;
    Ok(())
}
