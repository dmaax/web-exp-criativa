use rocket::{post, http::Status, serde::json::Json};
use serde::Deserialize;
use diesel::prelude::*;
use crate::schema::usuarios;
use crate::login_db::conectar_escritor_leitor;
use crate::mail;
use crate::models::Usuario;

#[derive(Deserialize)]
pub struct EsqueciSenhaRequest {
    pub email: String,
}

#[post("/esqueci_senha", format = "json", data = "<req>")]
pub async fn esqueci_senha(req: Json<EsqueciSenhaRequest>) -> Result<Status, Status> {
    let mut conn = conectar_escritor_leitor();
    let result = usuarios::dsl::usuarios
        .filter(usuarios::dsl::email.eq(&req.email))
        .first::<Usuario>(&mut conn)
        .optional();

    match result {
        Ok(Some(_user)) => { 

            let _ = mail::send_email_senha(&req.email);

            Ok(Status::Ok)
        }
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
