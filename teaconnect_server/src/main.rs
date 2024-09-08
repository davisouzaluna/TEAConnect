use teaconnect_server::alunos::*;
use teaconnect_server::connection::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = create_connection(ConnectionType::Pool)?;

    match connection {
        Connection::Pool(pool) => {
            // Cria um novo aluno
            create_aluno(&pool, "João", "2000-01-01", "Diagnóstico A")?;

            // Lê e imprime todos os alunos
            let alunos = read_alunos(&pool)?;
            for aluno in alunos {
                println!("{:?}", aluno);
            }

            // Atualiza o aluno com ID 1
            update_aluno(&pool, 1, "João Atualizado", "2000-01-01", "Diagnóstico Atualizado")?;

            // Deleta o aluno com ID 1
            delete_aluno(&pool, 1)?;
        }
        _ => println!("Conexão única não suportada para pool."),
    }

    Ok(())
}
