<?php
include 'db.php';

// Excluir aluno
if (isset($_GET['delete_aluno'])) {
    $id = $_GET['delete_aluno'];

    $sql = "DELETE FROM Alunos WHERE ID = ?";
    $stmt = $pdo->prepare($sql);
    $stmt->execute([$id]);

    echo "Aluno excluído com sucesso!";
}

// Excluir usuário
if (isset($_GET['delete_usuario'])) {
    $id = $_GET['delete_usuario'];

    $sql = "DELETE FROM Usuarios WHERE ID = ?";
    $stmt = $pdo->prepare($sql);
    $stmt->execute([$id]);

    echo "Usuário excluído com sucesso!";
}
?>
<a href="?delete_aluno=1">Excluir Aluno com ID 1</a><br>
<a href="?delete_usuario=1">Excluir Usuário com ID 1</a>
