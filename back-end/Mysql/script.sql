create database if not exists TEAConnect;
use TEAConnect;

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
    TipoInformacao VARCHAR(100) NOT NULL,  -- Tipo de informação (por exemplo, 'Evolução', 'Progresso', etc.)
    Valor FLOAT NOT NULL,  -- Valor quantitativo da informação
    Data DATE NOT NULL,
    FOREIGN KEY (ID_Aluno) REFERENCES Alunos(ID),
    FOREIGN KEY (ID_Usuario) REFERENCES Usuarios(ID),
    FOREIGN KEY (ID_Planilha) REFERENCES Planilhas(ID)
);
