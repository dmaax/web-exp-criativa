use rocket::serde::{Deserialize, json::Json};
use rocket::post;
use diesel::prelude::*;
use bcrypt::verify; // Certifique-se de que a dependência bcrypt está no Cargo.toml
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
pub fn verificar_login(credenciais: Json<CredenciaisLogin>) -> Json<bool> {
    let mut conn = conectar_escritor_leitor();

    let resultado = usuarios
        .filter(email.eq(&credenciais.email))
        .first::<Usuario>(&mut conn)
        .optional();

    match resultado {
        Ok(Some(usuario)) => {
            println!("Senha hash no banco: {}", usuario.cep);
            println!("Senha fornecida: {}", credenciais.senha);

            if credenciais.senha == usuario.cep {
                Json(true)
            } else {
                eprintln!("Erro: As senhas não coincidem.");
                Json(false)
            }
        },
        _ => Json(false),
    }
}
