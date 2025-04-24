use rocket::serde::{Deserialize, json::Json};
use rocket::post;
use diesel::prelude::*;
use crate::schema::usuarios::dsl::*;
use crate::login_db::conectar_escritor_leitor;
use crate::models::Usuario;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CredenciaisLogin {
    pub email: String,
    pub senha: String,
}

#[post("/login", format = "json", data = "<credenciais>")]
pub fn verificar_login(credenciais: Json<CredenciaisLogin>) -> Option<Json<String>> {
    let mut conn = conectar_escritor_leitor();

    // Procura usuário pelo e-mail
    let resultado = usuarios
        .filter(email.eq(&credenciais.email))
        .first::<Usuario>(&mut conn)
        .optional();

    match resultado {
        Ok(Some(usuario)) => {
            if usuario.senha_hash == credenciais.senha {
                let codigo_mfa = usuario.codigo_2fa.clone(); 
                Some(Json(codigo_mfa))
            } else {
                None
            }
        },
        _ => None,
    }
}
