<?php
include 'db.php';

// Inserir novo aluno
if (isset($_POST['create_aluno'])) {
    $nome = $_POST['nome'];
    $dataNascimento = $_POST['dataNascimento'];
    $diagnostico = $_POST['diagnostico'];

    $sql = "INSERT INTO Alunos (Nome, DataNascimento, DiagnosticoTEA) VALUES (?, ?, ?)";
    $stmt = $pdo->prepare($sql);
    $stmt->execute([$nome, $dataNascimento, $diagnostico]);

    echo "Aluno cadastrado com sucesso!";
}

// Inserir novo usuário
if (isset($_POST['create_usuario'])) {
    $nome = $_POST['nome'];
    $email = $_POST['email'];
    $tipo = $_POST['tipo'];

    $sql = "INSERT INTO Usuarios (Nome, Email, Tipo) VALUES (?, ?, ?)";
    $stmt = $pdo->prepare($sql);
    $stmt->execute([$nome, $email, $tipo]);

    echo "Usuário cadastrado com sucesso!";
}
?>
<form method="POST">
    <h3>Cadastrar Aluno</h3>
    Nome: <input type="text" name="nome"><br>
    Data de Nascimento: <input type="date" name="dataNascimento"><br>
    Diagnóstico TEA: <input type="text" name="diagnostico"><br>
    <button type="submit" name="create_aluno">Cadastrar Aluno</button>
</form>

<form method="POST">
    <h3>Cadastrar Usuário</h3>
    Nome: <input type="text" name="nome"><br>
    E-mail: <input type="email" name="email"><br>
    Tipo: <select name="tipo">
        <option value="professor">Professor</option>
        <option value="pai">Pai</option>
        <option value="profissional">Profissional</option>
    </select><br>
    <button type="submit" name="create_usuario">Cadastrar Usuário</button>
</form>
