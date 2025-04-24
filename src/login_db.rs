use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;

pub fn conectar_root() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DB_ROOT_URL").expect("DATABASE_URL não configurado");
    PgConnection::establish(&database_url).expect(&format!("Falha ao conectar a {}", database_url))
}

pub fn conectar_escritor() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL_ESCRITOR não configurado");
    PgConnection::establish(&database_url).expect(&format!("Falha ao conectar a {}", database_url))
}

pub fn conectar_editor() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DB_EDITOR_URL").expect("DATABASE_URL_EDITOR não configurado");
    PgConnection::establish(&database_url).expect(&format!("Falha ao conectar a {}", database_url))
}
