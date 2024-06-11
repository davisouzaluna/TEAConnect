require('dotenv').config();
const express = require('express');
const bodyParser = require('body-parser');
const app = express();
const PORT = process.env.PORT || 3000;

const routes = require('./routes');

app.use(bodyParser.json());
app.use(express.static('public'));
app.use('/', routes);
app.use((req, res, next) => {
    res.status(404).send("Página não encontrada");
  });

app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});
