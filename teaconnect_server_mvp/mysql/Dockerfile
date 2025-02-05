FROM mysql:latest

ENV MYSQL_ROOT_PASSWORD=root
ENV MYSQL_DATABASE=TEAConnect

COPY db-init.sql /docker-entrypoint-initdb.d/
