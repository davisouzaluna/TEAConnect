<?php
include 'db.php';

// Verifica se o ID foi passado via GET
if (isset($_GET['id'])) {
    $id = intval($_GET['id']);

    // Consulta a planilha no banco de dados
    $sql = "SELECT Alunos.Nome AS Aluno, Usuarios.Nome AS Usuario, Planilhas.Data, Planilhas.Conteudo 
            FROM Planilhas
            JOIN Alunos ON Planilhas.ID_Aluno = Alunos.ID
            JOIN Usuarios ON Planilhas.ID_Usuario = Usuarios.ID
            WHERE Planilhas.ID = :ID";
    $stmt = $pdo->prepare($sql);
    $stmt->bindParam(':ID', $id, PDO::PARAM_INT);
    $stmt->execute();
    $planilha = $stmt->fetch(PDO::FETCH_ASSOC);

    if ($planilha) {
        // Define o nome do arquivo
        $filename = "planilha_" . $id . ".csv";

        // Define os cabeçalhos para o download do arquivo
        header("Content-Type: text/csv; charset=utf-8");
        header("Content-Disposition: attachment; filename=$filename");

        // Cria um ponteiro de saída
        $output = fopen('php://output', 'w');

        // Escreve os títulos das colunas
        fputcsv($output, ['Aluno', 'Usuário', 'Data', 'Conteúdo']);

        // Escreve os dados da planilha
        fputcsv($output, [$planilha['Aluno'], $planilha['Usuario'], $planilha['Data'], $planilha['Conteudo']]);

        // Fecha o ponteiro de saída
        fclose($output);
        exit;
    } else {
        echo "Planilha não encontrada.";
    }
} else {
    echo "ID não especificado.";
}
?>
