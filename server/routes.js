const express = require('express');
const router = express.Router();
const connection = require('./db');

// Rota para o login de professores (Usuários)
router.post('/professores/login', (req, res) => {
    const { email } = req.body;
    connection.query('SELECT * FROM Usuarios WHERE Email = ? AND Tipo = "professor"', [email], (err, results) => {
        if (err) {
            res.status(500).json({ error: err.message });
            return;
        }
        if (results.length === 0) {
            res.status(404).json({ message: 'Usuário não encontrado ou não é um professor.' });
            return;
        }
        res.json(results[0]);
    });
});

// Rota para o login de alunos
router.post('/alunos/login', (req, res) => {
    const { nome } = req.body;
    connection.query('SELECT * FROM Alunos WHERE Nome = ?', [nome], (err, results) => {
        if (err) {
            res.status(500).json({ error: err.message });
            return;
        }
        if (results.length === 0) {
            res.status(404).json({ message: 'Aluno não encontrado.' });
            return;
        }
        res.json(results[0]);
    });
});
router.post('/cadastro', (req, res) => {
    const { nome, email, tipo } = req.body;

    // Verificar se o usuário já existe
    connection.query('SELECT * FROM Usuarios WHERE Email = ?', [email], (err, results) => {
        if (err) {
            res.status(500).json({ error: err.message });
            return;
        }
        if (results.length > 0) {
            res.status(409).json({ message: 'Usuário já cadastrado.' });
            return;
        }

        // Se o usuário não existir, inserir na tabela
        connection.query('INSERT INTO Usuarios (Nome, Email, Tipo) VALUES (?, ?, ?)', [nome, email, tipo], (err, result) => {
            if (err) {
                res.status(500).json({ error: err.message });
                return;
            }
            res.status(201).json({ message: 'Usuário cadastrado com sucesso.' });
        });
    });
});
module.exports = router;
