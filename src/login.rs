use rocket::serde::{Deserialize, json::Json};
use rocket::post;
use diesel::prelude::*;
use crate::schema::usuarios::dsl::*;
use crate::login_db::conectar_escritor_leitor;
use crate::models::Usuario;
use rocket::http::{Cookie, CookieJar};
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
//entrada do json
pub struct CredenciaisLogin { 
    pub email: String,
    pub senha: String,
} 


#[post("/login", format = "json", data = "<credenciais>")]
// pq precisa passar como parametro na func se n existe cookie antes? simples
// vms comparar com uma caixinha de cookie, q a gente quer colocar um cookie dentro dela
// e depois pegar esse cookie, e ver se ele existe ou n
// mas eu preciso da caixinha de cookie para colocar o cookie dentro dela
// ent oq estamos passando como parametro é a caixinha de cookie para a gente poder ler e add, dai vira putaria e a gente faz oq quiser
pub fn verificar_login(credenciais: Json<CredenciaisLogin>, cookies: &CookieJar<'_>) -> Json<bool> {
    let mut conn = conectar_escritor_leitor();
    // SELECT * FROM usuarios WHERE email = email-entrada
    let resultado = usuarios
        .filter(email.eq(&credenciais.email))
        //pega o primeiro usuario que tem o email
        .first::<Usuario>(&mut conn) 
        //optional retorna um resultado com o usuario ou None se nao encontrar
        .optional();

    match resultado {
        Ok(Some(usuario)) => {
            if credenciais.senha == usuario.senha_hash {
                // Cria um cookie com o ID do usuário 
                cookies.add(Cookie::new("user_id", usuario.id.to_string()));
                //retorna true se existe e vai para o front
                // e la ele trata esse retorno
                Json(true)
            } else {
                Json(false)
            }
        },
        _ => Json(false),
    }
}