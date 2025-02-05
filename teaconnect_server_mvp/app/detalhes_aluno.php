<?php
include 'db.php';

// Buscar todos os alunos
$sqlAlunos = "SELECT * FROM Alunos";
$stmtAlunos = $pdo->prepare($sqlAlunos);
$stmtAlunos->execute();
$alunos = $stmtAlunos->fetchAll(PDO::FETCH_ASSOC);

if (empty($alunos)) {
    echo "Não há alunos cadastrados!";
    exit;
}
?>

<!DOCTYPE html>
<html lang="pt-br">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Detalhes dos Alunos</title>
</head>
<body>
    <h1>Detalhes dos Alunos</h1>

    <?php foreach ($alunos as $aluno): ?>
        <h2>Aluno: <?= htmlspecialchars($aluno['Nome']) ?></h2>
        <p><strong>Data de Nascimento:</strong> <?= htmlspecialchars($aluno['DataNascimento']) ?></p>
        <p><strong>Diagnóstico TEA:</strong> <?= htmlspecialchars($aluno['DiagnosticoTEA']) ?></p>

        <?php
        // Buscar planilhas do aluno
        $sqlPlanilhas = "SELECT Planilhas.ID, Planilhas.Data, Planilhas.Conteudo 
                         FROM Planilhas
                         WHERE Planilhas.ID_Aluno = :aluno_id";
        $stmtPlanilhas = $pdo->prepare($sqlPlanilhas);
        $stmtPlanilhas->bindParam(':aluno_id', $aluno['ID'], PDO::PARAM_INT);
        $stmtPlanilhas->execute();
        $planilhas = $stmtPlanilhas->fetchAll(PDO::FETCH_ASSOC);
        ?>

        <?php if (!empty($planilhas)): ?>
            <h3>Planilhas:</h3>
            <ul>
                <?php foreach ($planilhas as $planilha): ?>
                    <li>
                        <strong>Data:</strong> <?= htmlspecialchars($planilha['Data']) ?> |
                        <a href="gerar_planilha.php?id=<?= $planilha['ID'] ?>">Baixar Planilha CSV</a>
                    </li>
                <?php endforeach; ?>
            </ul>
        <?php else: ?>
            <p><strong>Este aluno não possui planilhas.</strong></p>
        <?php endif; ?>

        <hr>
    <?php endforeach; ?>
</body>
</html>
