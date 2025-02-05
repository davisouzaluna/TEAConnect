<?php
include 'db.php';

// Atualizar aluno
if (isset($_POST['update_aluno'])) {
    $id = $_POST['id'];
    $nome = $_POST['nome'];
    $dataNascimento = $_POST['dataNascimento'];
    $diagnostico = $_POST['diagnostico'];

    $sql = "UPDATE Alunos SET Nome = ?, DataNascimento = ?, DiagnosticoTEA = ? WHERE ID = ?";
    $stmt = $pdo->prepare($sql);
    $stmt->execute([$nome, $dataNascimento, $diagnostico, $id]);

    echo "Aluno atualizado com sucesso!";
}

// Atualizar usuário
if (isset($_POST['update_usuario'])) {
    $id = $_POST['id'];
    $nome = $_POST['nome'];
    $email = $_POST['email'];
    $tipo = $_POST['tipo'];

    $sql = "UPDATE Usuarios SET Nome = ?, Email = ?, Tipo = ? WHERE ID = ?";
    $stmt = $pdo->prepare($sql);
    $stmt->execute([$nome, $email, $tipo, $id]);

    echo "Usuário atualizado com sucesso!";
}
?>
<form method="POST">
    <h3>Atualizar Aluno</h3>
    ID: <input type="text" name="id"><br>
    Nome: <input type="text" name="nome"><br>
    Data de Nascimento: <input type="date" name="dataNascimento"><br>
    Diagnóstico TEA: <input type="text" name="diagnostico"><br>
    <button type="submit" name="update_aluno">Atualizar Aluno</button>
</form>

<form method="POST">
    <h3>Atualizar Usuário</h3>
    ID: <input type="text" name="id"><br>
    Nome: <input type="text" name="nome"><br>
    E-mail: <input type="email" name="email"><br>
    Tipo: <select name="tipo">
        <option value="professor">Professor</option>
        <option value="pai">Pai</option>
        <option value="profissional">Profissional</option>
    </select><br>
    <button type="submit" name="update_usuario">Atualizar Usuário</button>
</form>
