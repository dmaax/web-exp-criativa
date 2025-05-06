#[macro_use] extern crate rocket;
use dotenv::dotenv;
use rocket::response::Redirect;
use rocket::fs::{FileServer, NamedFile};
use std::env;
use std::path::Path;


mod cpf;
mod mail;
mod autenticador;
mod login_db;
mod criacao_conta;
mod models;
mod schema;
mod login;
mod cria_conta_cartao;

#[get("/")]
fn root() -> Redirect {
    Redirect::to("/static/html/index.html")
}

#[get("/<file>")]
async fn html_files(file: &str) -> Option<NamedFile> {
    let path: String = format!("static/html/{}.html", file);
    NamedFile::open(Path::new(&path)).await.ok()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let _smtp_user: Box<str> = env::var("SMTP_USER").expect("SMTP_USER não configurado").into();
    let _smtp_password: Box<str> = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD não configurado").into();

    rocket::build()
        .mount("/", routes![root, html_files, cpf::vcpf, autenticador::vcod, criacao_conta::criar_conta, login::verificar_login])


        .mount("/static", FileServer::from("static"))
}