# API - Rust

# Cargo Run

# curl -X POST http://127.0.0.1:3000/users -H "Content-Type: application/json" -d '{"name": "Lucas", "email": "lucas@email.com"}'

# curl http://127.0.0.1:3000/users

Base de Dados {
    CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);

}
