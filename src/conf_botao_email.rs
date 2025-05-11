use rocket::{http::Status, serde::json::Json};
use rocket::serde::Deserialize;
use crate::autenticador::valida_codigo_autenticador;
use crate::login_db::conectar_escritor_leitor;
use crate::models::Usuario;
use crate::schema::usuarios::dsl::*;
use diesel::prelude::*;

#[derive(Deserialize)]
pub struct EntradaVerificaMfa {
    pub cpf: String,
    pub codigo: String,
}

#[post("/verificaEmailAndCriaContaBanco", format = "json", data = "<entrada>")]
pub async fn veri_email_e_cria_conta_usuario_banco(entrada: Json<EntradaVerificaMfa>) -> Result<Json<bool>, Status> {
    let mut conn = conectar_escritor_leitor();

    let usuario_result = usuarios
        .filter(cpf.eq(&entrada.cpf))
        .first::<Usuario>(&mut conn)
        .optional();

    if let Ok(Some(usuario)) = usuario_result {
        let saida_codigo = valida_codigo_autenticador(&usuario.codigo_2fa);
        if entrada.codigo.trim() == saida_codigo {
            return Ok(Json(true));
        }
    }

    Ok(Json(false))
}
