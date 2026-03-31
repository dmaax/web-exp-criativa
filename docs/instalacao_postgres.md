# Instalação e configuração do PostgreSQL para o projeto

Este guia reúne os passos necessários para criar o banco, configurar os usuários e rodar as migrações com Diesel.

## 1. Instalar o PostgreSQL e dependências

No Ubuntu/Debian:

```bash
sudo apt update
sudo apt install postgresql postgresql-contrib libpq-dev pkg-config libssl-dev gcc
```

## 2. Instalar o Diesel CLI com suporte a PostgreSQL

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## 3. Criar o banco de dados e os usuários PostgreSQL

Execute o `psql` como usuário postgres:

```bash
sudo -u postgres psql
```

Dentro do `psql`:

```sql
CREATE USER root_app WITH PASSWORD 'senha_root';
CREATE USER escritor_app WITH PASSWORD 'senha_escritor';
CREATE USER editor_app WITH PASSWORD 'senha_editor';

CREATE DATABASE projeto_rust OWNER root_app;
\c projeto_rust

GRANT ALL PRIVILEGES ON DATABASE projeto_rust TO root_app;
GRANT CONNECT ON DATABASE projeto_rust TO escritor_app;
GRANT CONNECT ON DATABASE projeto_rust TO editor_app;

ALTER SCHEMA public OWNER TO escritor_app;
GRANT ALL ON SCHEMA public TO escritor_app;
```

Depois:

```sql
\q
```

## 4. Criar o arquivo `.env` na raiz do projeto

Exemplo de `.env`:

```env
DB_ROOT_URL=postgres://root_app:senha_root@localhost/projeto_rust
DB_ESCRITOR_URL=postgres://escritor_app:senha_escritor@localhost/projeto_rust
DB_EDITOR_URL=postgres://editor_app:senha_editor@localhost/projeto_rust
DATABASE_URL=postgres://escritor_app:senha_escritor@localhost/projeto_rust
```

### Observações importantes

- `DATABASE_URL` é usado por `src/login_db.rs` em `conectar_escritor_leitor()`.
- `DB_EDITOR_URL` é usado por `src/login_db.rs` em `conectar_editor()`.
- `DB_ROOT_URL` fica disponível para administração, mas não é obrigatório para o uso normal da aplicação.

## 5. Ajustar `diesel.toml` se necessário

O arquivo `diesel.toml` deve apontar para o diretório de migrações do projeto. No root do repositório, o correto é:

```toml
[migrations_directory]
dir = "migrations"
```

Se o arquivo ainda não existir ou estiver apontando para um caminho errado, atualize-o.

## 6. Inicializar Diesel e criar a migração

Execute:

```bash
diesel setup
diesel migration generate criar_usuarios
```

Isso cria o diretório `migrations/<timestamp>_criar_usuarios`.

## 7. Preencher o arquivo `up.sql`

No arquivo `migrations/<timestamp>_criar_usuarios/up.sql`, coloque o SQL abaixo.

```sql
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

-- Tabela de contas
CREATE TABLE contas (
    id SERIAL PRIMARY KEY,
    usuario_id INTEGER NOT NULL,
    saldo VARCHAR(20) NOT NULL DEFAULT '0.00',
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE
);

GRANT SELECT, INSERT ON contas TO escritor_app;
GRANT SELECT, UPDATE ON contas TO editor_app;
REVOKE DELETE ON contas FROM PUBLIC;

-- Tabela de cartões
CREATE TABLE cartoes (
    id SERIAL PRIMARY KEY,
    conta_id INTEGER NOT NULL,
    numero_cartao VARCHAR(30) NOT NULL UNIQUE,
    data_cartao VARCHAR(8) NOT NULL,
    codigo_cartao VARCHAR(3) NOT NULL,
    saldo_disponivel VARCHAR(20) NOT NULL DEFAULT '10000.00',
    saldo_usado VARCHAR(20) NOT NULL DEFAULT '0.00',
    FOREIGN KEY (conta_id) REFERENCES contas(id) ON DELETE CASCADE
);

GRANT SELECT, INSERT ON cartoes TO escritor_app;
GRANT SELECT, UPDATE ON cartoes TO editor_app;
REVOKE DELETE ON cartoes FROM PUBLIC;

-- Tabela de empréstimos
CREATE TABLE emprestimos (
    id SERIAL PRIMARY KEY,
    conta_id INTEGER NOT NULL,
    valor_disponivel VARCHAR(20) NOT NULL DEFAULT '0.00',
    valor_emprestado VARCHAR(20) NOT NULL DEFAULT '0.00',
    FOREIGN KEY (conta_id) REFERENCES contas(id) ON DELETE CASCADE
);

GRANT SELECT, INSERT ON emprestimos TO escritor_app;
GRANT SELECT, UPDATE ON emprestimos TO editor_app;
REVOKE DELETE ON emprestimos FROM PUBLIC;

-- Tabela de extratos
CREATE TABLE extratos (
    id SERIAL PRIMARY KEY,
    conta_id INTEGER NOT NULL,
    nome_compra VARCHAR(100) NOT NULL,
    valor VARCHAR(20) NOT NULL,
    FOREIGN KEY (conta_id) REFERENCES contas(id) ON DELETE CASCADE
);

GRANT SELECT, INSERT ON extratos TO escritor_app;
GRANT SELECT, UPDATE ON extratos TO editor_app;
REVOKE DELETE ON extratos FROM PUBLIC;

GRANT SELECT, INSERT, UPDATE ON usuarios TO escritor_app;
GRANT SELECT, INSERT, UPDATE ON contas TO escritor_app;
GRANT SELECT, INSERT, UPDATE ON cartoes TO escritor_app;
GRANT SELECT, INSERT, UPDATE ON emprestimos TO escritor_app;
GRANT SELECT, INSERT, UPDATE ON extratos TO escritor_app;
```

## 8. Executar a migração

```bash
diesel migration run
```

## 9. Conferir se o banco está funcionando

Use o `psql` para verificar as tabelas:

```bash
sudo -u postgres psql -d projeto_rust -c "\dt"
```

E, para ver dados em `usuarios`:

```bash
sudo -u postgres psql -d projeto_rust -c "SELECT * FROM usuarios;"
```

## 10. Dica final

Se `diesel setup` falhar por falta de permissão, use temporariamente `DATABASE_URL` com `root_app` para inicializar o banco e depois volte para `escritor_app`.
