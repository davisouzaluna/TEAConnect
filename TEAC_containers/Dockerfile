# Usar imagem oficial do MySQL
FROM mysql:8.0

# Definir variáveis de ambiente para configurar o MySQL
ENV MYSQL_ROOT_PASSWORD=root_password
ENV MYSQL_DATABASE=TEAConnect
ENV MYSQL_USER=usuario
ENV MYSQL_PASSWORD=senha

# Copiar o script SQL para o diretório de inicialização do MySQL
COPY ./init.sql /docker-entrypoint-initdb.d/

# Expor a porta 3306 (porta padrão do MySQL)
EXPOSE 3306
