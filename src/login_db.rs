use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;

// aq é simples, ele faz as conexões com o banco de dados, com usuario diferentes, q foi pedido pelo lino, coloquei o root aq so por colocar msm
// acho q n vms usar, mas ta ai 

pub fn conectar_root() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DB_ROOT_URL").expect("DATABASE_URL não configurado");
    PgConnection::establish(&database_url).expect(&format!("Falha ao conectar a {}", database_url))
}

pub fn conectar_escritor_leitor() -> PgConnection { // e leitor
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL_ESCRITOR não configurado");
    PgConnection::establish(&database_url).expect(&format!("Falha ao conectar a {}", database_url))
}

pub fn conectar_editor() -> PgConnection { // e leitor
    dotenv().ok();
    let database_url = env::var("DB_EDITOR_URL").expect("DATABASE_URL_EDITOR não configurado");
    PgConnection::establish(&database_url).expect(&format!("Falha ao conectar a {}", database_url))
}
