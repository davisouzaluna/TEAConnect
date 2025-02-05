<?php
require 'db.php';

header('Content-Type: application/json');

$stmt = $pdo->query("SELECT * FROM Alunos");
$alunos = $stmt->fetchAll();

echo json_encode($alunos);
