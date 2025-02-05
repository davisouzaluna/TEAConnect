CREATE DATABASE IF NOT EXISTS TEAConnect;
USE TEAConnect;

-- Tabela Alunos
CREATE TABLE IF NOT EXISTS Alunos (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    Nome VARCHAR(100) NOT NULL,
    DataNascimento DATE NOT NULL,
    DiagnosticoTEA VARCHAR(100) NOT NULL
);

-- Tabela Usuários
CREATE TABLE IF NOT EXISTS Usuarios (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    Nome VARCHAR(100) NOT NULL,
    Email VARCHAR(100) NOT NULL UNIQUE,
    Tipo ENUM('professor', 'pai', 'profissional') NOT NULL
);

-- Tabela Planilhas
CREATE TABLE IF NOT EXISTS Planilhas (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    ID_Aluno INT NOT NULL,
    ID_Usuario INT NOT NULL,
    Data DATE NOT NULL,
    Conteudo TEXT NOT NULL,
    FOREIGN KEY (ID_Aluno) REFERENCES Alunos(ID),
    FOREIGN KEY (ID_Usuario) REFERENCES Usuarios(ID)
);

-- Tabela MensagensFórum
CREATE TABLE IF NOT EXISTS MensagensForum (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    ID_Usuario INT NOT NULL,
    Conteudo TEXT NOT NULL,
    Data DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ID_Usuario) REFERENCES Usuarios(ID)
);

-- Tabela CursosOnline
CREATE TABLE IF NOT EXISTS CursosOnline (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    Nome VARCHAR(100) NOT NULL,
    Descricao TEXT NOT NULL,
    Link VARCHAR(255) NOT NULL
);

-- Tabela MateriaisDidáticos
CREATE TABLE IF NOT EXISTS MateriaisDidaticos (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    Titulo VARCHAR(100) NOT NULL,
    Descricao TEXT NOT NULL,
    Arquivo VARCHAR(255) NOT NULL
);

-- Tabela InformacoesQuantitativas
CREATE TABLE IF NOT EXISTS InformacoesQuantitativas (
    ID INT AUTO_INCREMENT PRIMARY KEY,
    ID_Aluno INT NOT NULL,
    ID_Usuario INT NOT NULL,
    ID_Planilha INT NOT NULL,
    TipoInformacao VARCHAR(100) NOT NULL,
    Valor FLOAT NOT NULL,
    Data DATE NOT NULL,
    FOREIGN KEY (ID_Aluno) REFERENCES Alunos(ID),
    FOREIGN KEY (ID_Usuario) REFERENCES Usuarios(ID),
    FOREIGN KEY (ID_Planilha) REFERENCES Planilhas(ID)
);

-- Inserindo dados na tabela Usuarios
INSERT INTO Usuarios (Nome, Email, Tipo) VALUES
('Alice', 'alice@example.com', 'professor'),
('Bob', 'bob@example.com', 'pai'),
('Carlos', 'carlos@example.com', 'profissional');

-- Inserindo dados na tabela Alunos
INSERT INTO Alunos (Nome, DataNascimento, DiagnosticoTEA) VALUES
('João', '2010-05-15', 'Nível 1'),
('Maria', '2012-09-20', 'Nível 2');

-- Inserindo dados na tabela CursosOnline
INSERT INTO CursosOnline (Nome, Descricao, Link) VALUES
('Curso TEA para Professores', 'Curso completo sobre TEA para educadores', 'https://example.com/curso1'),
('Curso TEA para Pais', 'Curso de apoio para pais de crianças com TEA', 'https://example.com/curso2');
