-- Your SQL goes here
CREATE TABLE usuarios (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    email VARCHAR(150) NOT NULL UNIQUE,
    cpf VARCHAR(12) NOT NULL UNIQUE,
    data_nascimento VARCHAR(10) NOT NULL,
    telefone VARCHAR(16) NOT NULL,
    senha_hash TEXT NOT NULL,
    cep VARCHAR(9) NOT NULL,
    codigo_2fa VARCHAR(32) NOT NULL
);

GRANT SELECT, INSERT ON usuarios TO escritor_app;

GRANT SELECT, UPDATE ON usuarios TO editor_app;

REVOKE DELETE ON usuarios FROM PUBLIC;