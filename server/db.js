require('dotenv').config();
const mysql = require('mysql2');
const fs = require('fs');
const path = require('path');

const connection = mysql.createConnection({
    host: process.env.DB_HOST,
    user: process.env.DB_USER,
    password: process.env.DB_PASSWORD,
    port: process.env.DB_PORT,
    multipleStatements: true  // Permite executar múltiplas instruções SQL ao mesmo tempo
});

connection.connect((err) => {

    console.log('DB_HOST:', process.env.DB_HOST);
console.log('DB_USER:', process.env.DB_USER);
console.log('DB_PASSWORD:', process.env.DB_PASSWORD);
console.log('DB_PORT:', process.env.DB_PORT);
console.log('DB_NAME:', process.env.DB_NAME);

    if (err) {
        console.error('Error connecting to the database:', err);
        return;
    }
    console.log('Connected to the MySQL database.');

    const initSqlPath = path.join(__dirname, '..', 'BD', '../back-end/Mysql/script.sql');
    const initSql = fs.readFileSync(initSqlPath, 'utf-8');

    connection.query(initSql, (err, results) => {
        if (err) {
            console.error('Error executing init SQL:', err);
            return;
        }
        console.log('Database initialized.');
        connection.changeUser({database: process.env.DB_NAME}, (err) => {
            if (err) {
                console.error('Error changing database:', err);
                return;
            }
            console.log('Switched to TEAConnect database.');
        });
    });
});



module.exports = connection;
