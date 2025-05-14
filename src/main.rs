#[macro_use] extern crate rocket;
use dotenv::dotenv;
use rocket::response::Redirect;
use rocket::fs::{FileServer, NamedFile};
use std::env;
use std::path::Path;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;


mod cpf;
mod mail;
mod autenticador;
mod login_db;
mod criacao_conta;
mod models;
mod schema;
mod login;
mod conf_botao_email;
mod cria_cartao;
mod newpasswd;
mod account;
mod card_pg;
mod altera_senha_esqueci_email;
mod esqueci_senha_arquivo;
mod sessao;



pub struct SessaoUsuario(pub i32);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessaoUsuario {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(cookie) = req.cookies().get("sessao_token") {
            let ip = req.client_ip().map(|ip| ip.to_string()).unwrap_or_default();
            let user_agent = req.headers().get_one("User-Agent").unwrap_or("").to_string();
            if let Some(user_id) = sessao::validar_sessao(cookie.value(), &ip, &user_agent) {
                return Outcome::Success(SessaoUsuario(user_id));
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}


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
        .mount("/", routes![
            root,
            html_files, 
            cpf::vcpf, 
            autenticador::vcod, 
            criacao_conta::criar_conta, 
            login::verificar_login,
            conf_botao_email::veri_email_e_cria_conta_usuario_banco,
            newpasswd::alterar_senha,
            account::dados_conta, // <- já usa SessaoUsuario
            account::depositar,   // <- ajuste para usar SessaoUsuario
            card_pg::listar_cartoes, // <- já usa SessaoUsuario
            card_pg::registrar_compra, // <- já usa SessaoUsuario
            account::pagar_divida, // <- ajuste para usar SessaoUsuario
            altera_senha_esqueci_email::alterar_senha_email,
            esqueci_senha_arquivo::esqueci_senha,])


        .mount("/static", FileServer::from("static"))
}