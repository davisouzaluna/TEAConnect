<?php
include 'db.php'; // Arquivo de conexão com o banco de dados

// Criar Planilha
if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['create'])) {
    $id_aluno = intval($_POST['ID_Aluno']);
    $id_usuario = intval($_POST['ID_Usuario']);
    $data = $_POST['Data'];
    $conteudo = $_POST['Conteudo'];

    $sql = "INSERT INTO Planilhas (ID_Aluno, ID_Usuario, Data, Conteudo) 
            VALUES (:ID_Aluno, :ID_Usuario, :Data, :Conteudo)";
    $stmt = $pdo->prepare($sql);
    $stmt->bindParam(':ID_Aluno', $id_aluno, PDO::PARAM_INT);
    $stmt->bindParam(':ID_Usuario', $id_usuario, PDO::PARAM_INT);
    $stmt->bindParam(':Data', $data);
    $stmt->bindParam(':Conteudo', $conteudo);

    if ($stmt->execute()) {
        echo "Planilha criada com sucesso!";
    } else {
        echo "Erro ao criar planilha!";
    }
}

// Atualizar Planilha
if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['update'])) {
    $id = intval($_POST['ID']);
    $conteudo = $_POST['Conteudo'];

    $sql = "UPDATE Planilhas SET Conteudo = :Conteudo WHERE ID = :ID";
    $stmt = $pdo->prepare($sql);
    $stmt->bindParam(':Conteudo', $conteudo);
    $stmt->bindParam(':ID', $id, PDO::PARAM_INT);

    if ($stmt->execute()) {
        echo "Planilha atualizada com sucesso!";
    } else {
        echo "Erro ao atualizar planilha!";
    }
}

// Excluir Planilha
if (isset($_GET['delete'])) {
    $id = intval($_GET['delete']);

    $sql = "DELETE FROM Planilhas WHERE ID = :ID";
    $stmt = $pdo->prepare($sql);
    $stmt->bindParam(':ID', $id, PDO::PARAM_INT);

    if ($stmt->execute()) {
        echo "Planilha excluída com sucesso!";
    } else {
        echo "Erro ao excluir planilha!";
    }
}
?>

<!DOCTYPE html>
<html lang="pt-br">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CRUD de Planilhas</title>
</head>
<body>

<h3>Criar Nova Planilha</h3>
<form method="POST">
    <label>Aluno:</label>
    <select name="ID_Aluno" required>
        <?php
        $sql = "SELECT ID, Nome FROM Alunos";
        foreach ($pdo->query($sql) as $row) {
            echo "<option value='{$row['ID']}'>{$row['Nome']}</option>";
        }
        ?>
    </select><br>

    <label>Usuário:</label>
    <select name="ID_Usuario" required>
        <?php
        $sql = "SELECT ID, Nome FROM Usuarios";
        foreach ($pdo->query($sql) as $row) {
            echo "<option value='{$row['ID']}'>{$row['Nome']}</option>";
        }
        ?>
    </select><br>

    <label>Data:</label>
    <input type="date" name="Data" required><br>

    <label>Conteúdo:</label>
    <textarea name="Conteudo" required></textarea><br>

    <button type="submit" name="create">Criar Planilha</button>
</form>

<hr>

<h3>Lista de Planilhas</h3>
<table border="1">
    <tr>
        <th>ID</th>
        <th>Aluno</th>
        <th>Usuário</th>
        <th>Data</th>
        <th>Conteúdo</th>
        <th>Ações</th>
    </tr>
    <?php
    $sql = "SELECT Planilhas.ID, Alunos.Nome AS Aluno, Usuarios.Nome AS Usuario, Planilhas.Data, Planilhas.Conteudo 
            FROM Planilhas
            JOIN Alunos ON Planilhas.ID_Aluno = Alunos.ID
            JOIN Usuarios ON Planilhas.ID_Usuario = Usuarios.ID";
    foreach ($pdo->query($sql) as $row) {
        echo "<tr>
                <td>{$row['ID']}</td>
                <td>{$row['Aluno']}</td>
                <td>{$row['Usuario']}</td>
                <td>{$row['Data']}</td>
                <td>{$row['Conteudo']}</td>
                <td>
                    <form method='POST' style='display:inline;'>
                        <input type='hidden' name='ID' value='{$row['ID']}'>
                        <input type='text' name='Conteudo' value='{$row['Conteudo']}' required>
                        <button type='submit' name='update'>Editar</button>
                    </form>
                    <a href='?delete={$row['ID']}' onclick='return confirm(\"Tem certeza que deseja excluir?\")'>Excluir</a>
                    <a href='gerar_planilha.php?id={$row['ID']}' target='_blank'>Baixar</a>
                </td>
              </tr>";
    }
    ?>
</table>

</body>
</html>