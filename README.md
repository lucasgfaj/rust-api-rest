API - Rust

Executando o Projeto
--------------------
Para rodar a API, utilize o seguinte comando:

cargo run

Endpoints
---------

Criar um usuário:
Faz uma requisição POST para criar um novo usuário.

curl -X POST http://127.0.0.1:3000/users \
     -H "Content-Type: application/json" \
     -d '{"name": "Lucas", "email": "lucas@email.com"}'

Listar usuários:
Retorna a lista de usuários cadastrados.

curl http://127.0.0.1:3000/users

Banco de Dados
-------------
Criação da tabela users no PostgreSQL:

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);
