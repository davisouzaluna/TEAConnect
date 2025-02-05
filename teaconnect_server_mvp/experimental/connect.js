import mysql from 'mysql2/promise';

const connection = await mysql.createConnection(
    {
        host: `localhost`,
        user: `root`,
        database: `TEAConnect`,
        port: 3308,
        password: `root_password`
    }
);

try{
    const[results,fields] = await connection.query(
        `SELECT * FROM Usuarios`
    );
    console.log(results);
    console.log(fields);

} catch(error){
    console.log(error)
}

try{
    const[results,fields] = await connection.query(
        `INSERT INTO Usuarios (nome, email) VALUES ('João', 'joao@gmail.com')`
    );
    console.log(results);
    console.log(fields);

} catch(error){
    console.log(error);
}