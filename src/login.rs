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
// aq ele verifica se o ser existe no banco 
#[post("/login", format = "json", data = "<credenciais>")]
pub fn verificar_login(credenciais: Json<CredenciaisLogin>) -> Json<bool> {
    let mut conn = conectar_escritor_leitor();

    let resultado = usuarios
        .filter(email.eq(&credenciais.email))
        .first::<Usuario>(&mut conn)
        .optional();

    match resultado {
        Ok(Some(usuario)) => {
            if credenciais.senha == usuario.senha_hash {
                Json(true)
            } else {
                Json(false)
            }
        },
        _ => Json(false),
    }
}
