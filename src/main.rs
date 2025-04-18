#[macro_use] extern crate rocket;
use dotenv::dotenv;
use rocket::response::Redirect;
use rocket::fs::{FileServer, NamedFile};
use std::env;
use std::path::Path;

mod cpf;
mod mail;
mod autenticador;

#[get("/")]
fn root() -> Redirect {
    Redirect::to("/static/html/index.html")
}

#[get("/<file>")]
async fn html_files(file: &str) -> Option<NamedFile> {
    let path = format!("static/html/{}.html", file);
    NamedFile::open(Path::new(&path)).await.ok()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let _smtp_user = env::var("SMTP_USER").expect("SMTP_USER não configurado");
    let _smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD não configurado");

    rocket::build()
        .mount("/", routes![root, html_files, mail::send_verification, cpf::vcpf, autenticador::vcod])


        .mount("/static", FileServer::from("static"))
}