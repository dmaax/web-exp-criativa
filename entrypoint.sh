#!/bin/sh
set -e

echo "Aplicando migrations do Diesel..."

./diesel migration run --database-url "$DATABASE_URL"

echo "Migrations aplicadas com sucesso! Iniciando aplicação..."

exec /app/web-exp-criativa
