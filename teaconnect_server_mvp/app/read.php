<?php
include 'db.php';

// Ler alunos com detalhes
$sql = "SELECT * FROM Alunos";
$stmt = $pdo->query($sql);
$alunos = $stmt->fetchAll();

echo "<h3>Alunos</h3>";
echo "<table border='1' cellpadding='10'>";
echo "<tr><th>ID</th><th>Nome</th><th>Data de Nascimento</th><th>Diagnóstico TEA</th><th>Detalhes</th></tr>";

foreach ($alunos as $aluno) {
    echo "<tr>";
    echo "<td>" . $aluno['ID'] . "</td>";
    echo "<td>" . $aluno['Nome'] . "</td>";
    echo "<td>" . $aluno['DataNascimento'] . "</td>";
    echo "<td>" . $aluno['DiagnosticoTEA'] . "</td>";
    echo "<td><a href='detalhes_aluno.php?aluno_id=" . $aluno['ID'] . "'>Ver Detalhes</a></td>";
    echo "</tr>";
}
echo "</table>";

// Ler usuários com detalhes
$sql = "SELECT * FROM Usuarios";
$stmt = $pdo->query($sql);
$usuarios = $stmt->fetchAll();

echo "<h3>Usuários</h3>";
echo "<table border='1' cellpadding='10'>";
echo "<tr><th>ID</th><th>Nome</th><th>Email</th><th>Tipo</th></tr>";

foreach ($usuarios as $usuario) {
    echo "<tr>";
    echo "<td>" . $usuario['ID'] . "</td>";
    echo "<td>" . $usuario['Nome'] . "</td>";
    echo "<td>" . $usuario['Email'] . "</td>";
    echo "<td>" . $usuario['Tipo'] . "</td>";
    echo "</tr>";
}
echo "</table>";

// Ler planilhas com associação aos alunos e usuários
$sql = "SELECT p.ID, p.Data, p.Conteudo, a.Nome AS Aluno, u.Nome AS Usuario 
        FROM Planilhas p
        JOIN Alunos a ON p.ID_Aluno = a.ID
        JOIN Usuarios u ON p.ID_Usuario = u.ID";
$stmt = $pdo->query($sql);
$planilhas = $stmt->fetchAll();

echo "<h3>Planilhas</h3>";
echo "<table border='1' cellpadding='10'>";
echo "<tr><th>ID</th><th>Data</th><th>Conteúdo</th><th>Aluno</th><th>Usuário</th></tr>";

foreach ($planilhas as $planilha) {
    echo "<tr>";
    echo "<td>" . $planilha['ID'] . "</td>";
    echo "<td>" . $planilha['Data'] . "</td>";
    echo "<td>" . $planilha['Conteudo'] . "</td>";
    echo "<td>" . $planilha['Aluno'] . "</td>";
    echo "<td>" . $planilha['Usuario'] . "</td>";
    echo "</tr>";
}
echo "</table>";

// Ler mensagens do fórum com o nome do usuário
$sql = "SELECT mf.ID, mf.Conteudo, mf.Data, u.Nome AS Usuario 
        FROM MensagensForum mf
        JOIN Usuarios u ON mf.ID_Usuario = u.ID";
$stmt = $pdo->query($sql);
$mensagens = $stmt->fetchAll();

echo "<h3>Mensagens do Fórum</h3>";
echo "<table border='1' cellpadding='10'>";
echo "<tr><th>ID</th><th>Conteúdo</th><th>Data</th><th>Usuário</th></tr>";

foreach ($mensagens as $mensagem) {
    echo "<tr>";
    echo "<td>" . $mensagem['ID'] . "</td>";
    echo "<td>" . $mensagem['Conteudo'] . "</td>";
    echo "<td>" . $mensagem['Data'] . "</td>";
    echo "<td>" . $mensagem['Usuario'] . "</td>";
    echo "</tr>";
}
echo "</table>";

// Ler cursos online
$sql = "SELECT * FROM CursosOnline";
$stmt = $pdo->query($sql);
$cursos = $stmt->fetchAll();

echo "<h3>Cursos Online</h3>";
echo "<table border='1' cellpadding='10'>";
echo "<tr><th>ID</th><th>Nome</th><th>Descrição</th><th>Link</th></tr>";

foreach ($cursos as $curso) {
    echo "<tr>";
    echo "<td>" . $curso['ID'] . "</td>";
    echo "<td>" . $curso['Nome'] . "</td>";
    echo "<td>" . $curso['Descricao'] . "</td>";
    echo "<td><a href='" . $curso['Link'] . "' target='_blank'>Visitar Curso</a></td>";
    echo "</tr>";
}
echo "</table>";

// Ler materiais didáticos
$sql = "SELECT * FROM MateriaisDidaticos";
$stmt = $pdo->query($sql);
$materiais = $stmt->fetchAll();

echo "<h3>Materiais Didáticos</h3>";
echo "<table border='1' cellpadding='10'>";
echo "<tr><th>ID</th><th>Título</th><th>Descrição</th><th>Arquivo</th></tr>";

foreach ($materiais as $material) {
    echo "<tr>";
    echo "<td>" . $material['ID'] . "</td>";
    echo "<td>" . $material['Titulo'] . "</td>";
    echo "<td>" . $material['Descricao'] . "</td>";
    echo "<td><a href='" . $material['Arquivo'] . "' target='_blank'>Baixar</a></td>";
    echo "</tr>";
}
echo "</table>";

// Ler informações quantitativas com detalhes
$sql = "SELECT iq.ID, iq.TipoInformacao, iq.Valor, iq.Data, a.Nome AS Aluno, u.Nome AS Usuario, p.Data AS PlanilhaData
        FROM InformacoesQuantitativas iq
        JOIN Alunos a ON iq.ID_Aluno = a.ID
        JOIN Usuarios u ON iq.ID_Usuario = u.ID
        JOIN Planilhas p ON iq.ID_Planilha = p.ID";
$stmt = $pdo->query($sql);
$informacoes = $stmt->fetchAll();

echo "<h3>Informações Quantitativas</h3>";
echo "<table border='1' cellpadding='10'>";
echo "<tr><th>ID</th><th>Tipo de Informação</th><th>Valor</th><th>Data</th><th>Aluno</th><th>Usuário</th><th>Data da Planilha</th></tr>";

foreach ($informacoes as $informacao) {
    echo "<tr>";
    echo "<td>" . $informacao['ID'] . "</td>";
    echo "<td>" . $informacao['TipoInformacao'] . "</td>";
    echo "<td>" . $informacao['Valor'] . "</td>";
    echo "<td>" . $informacao['Data'] . "</td>";
    echo "<td>" . $informacao['Aluno'] . "</td>";
    echo "<td>" . $informacao['Usuario'] . "</td>";
    echo "<td>" . $informacao['PlanilhaData'] . "</td>";
    echo "</tr>";
}
echo "</table>";
?>
