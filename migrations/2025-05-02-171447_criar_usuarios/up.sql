-- Your SQL goes here
-- Tabela de usuários
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

-- Tabela de contas (ligada a usuários)
CREATE TABLE contas (
    id SERIAL PRIMARY KEY,
    usuario_id INTEGER NOT NULL,
    saldo VARCHAR(20) NOT NULL DEFAULT '0.00',
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE
);

GRANT SELECT, INSERT ON contas TO escritor_app;
GRANT SELECT, UPDATE ON contas TO editor_app;
REVOKE DELETE ON contas FROM PUBLIC;

-- Tabela de cartões (ligada a contas)
CREATE TABLE cartoes (
    id SERIAL PRIMARY KEY,
    conta_id INTEGER NOT NULL,
    numero_cartao VARCHAR(20) NOT NULL UNIQUE,
    saldo_disponivel VARCHAR(20) NOT NULL DEFAULT '0.00',
    saldo_usado VARCHAR(20) NOT NULL DEFAULT '0.00',
    FOREIGN KEY (conta_id) REFERENCES contas(id) ON DELETE CASCADE
);

GRANT SELECT, INSERT ON cartoes TO escritor_app;
GRANT SELECT, UPDATE ON cartoes TO editor_app;
REVOKE DELETE ON cartoes FROM PUBLIC;

-- Tabela de empréstimos (ligada a contas)
CREATE TABLE emprestimos (
    id SERIAL PRIMARY KEY,
    conta_id INTEGER NOT NULL,
    valor_disponivel VARCHAR(20) NOT NULL DEFAULT '0.00',
    FOREIGN KEY (conta_id) REFERENCES contas(id) ON DELETE CASCADE
);

GRANT SELECT, INSERT ON emprestimos TO escritor_app;
GRANT SELECT, UPDATE ON emprestimos TO editor_app;
REVOKE DELETE ON emprestimos FROM PUBLIC;

-- Tabela de extratos (ligada a contas)
CREATE TABLE extratos (
    id SERIAL PRIMARY KEY,
    conta_id INTEGER NOT NULL,
    nome_compra VARCHAR(100) NOT NULL,
    valor VARCHAR(20) NOT NULL,
    data_compra TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (conta_id) REFERENCES contas(id) ON DELETE CASCADE
);

GRANT SELECT, INSERT ON extratos TO escritor_app;
GRANT SELECT, UPDATE ON extratos TO editor_app;
REVOKE DELETE ON extratos FROM PUBLIC;