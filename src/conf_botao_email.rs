use rocket::{http::Status, serde::json::Json};
use rocket::serde::Deserialize;
use crate::autenticador::valida_codigo_autenticador;
use crate::cria_cartao::{cvs_cartao, numeros_cartao};
use crate::login_db::conectar_escritor_leitor;
use crate::models::Usuario;
use crate::schema::usuarios::dsl::*;
use diesel::prelude::*;
use crate::cria_cartao::data_validade_cartao;

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
            // Criar conta
            let conta_id: i32 = diesel::insert_into(crate::schema::contas::dsl::contas)
                .values((
                    crate::schema::contas::dsl::usuario_id.eq(usuario.id),
                    crate::schema::contas::dsl::saldo.eq("0.00"),
                ))
                .returning(crate::schema::contas::dsl::id)
                .get_result(&mut conn)
                .map_err(|_| Status::InternalServerError)?;

            // Criar cartão
            diesel::insert_into(crate::schema::cartoes::dsl::cartoes)
                .values((
                    crate::schema::cartoes::dsl::conta_id.eq(conta_id),
                    crate::schema::cartoes::dsl::numero_cartao.eq(numeros_cartao()),
                    crate::schema::cartoes::dsl::codigo_cartao.eq(cvs_cartao()),
                    crate::schema::cartoes::dsl::data_cartao.eq(data_validade_cartao()),
                    crate::schema::cartoes::dsl::saldo_disponivel.eq("10000.00"),
                    crate::schema::cartoes::dsl::saldo_usado.eq("0.00"),
                ))
                .execute(&mut conn)
                .map_err(|_| Status::InternalServerError)?;

            // Criar empréstimo
            diesel::insert_into(crate::schema::emprestimos::dsl::emprestimos)
                .values((
                    crate::schema::emprestimos::dsl::conta_id.eq(conta_id),
                    crate::schema::emprestimos::dsl::valor_disponivel.eq("0.00"),
                    crate::schema::emprestimos::dsl::valor_emprestado.eq("0.00"),
                ))
                .execute(&mut conn)
                .map_err(|_| Status::InternalServerError)?;

            // Criar extrato inicial
            diesel::insert_into(crate::schema::extratos::dsl::extratos)
                .values((
                    crate::schema::extratos::dsl::conta_id.eq(conta_id),
                    crate::schema::extratos::dsl::nome_compra.eq("Conta criada"),
                    crate::schema::extratos::dsl::valor.eq("0.00"),
                ))
                .execute(&mut conn)
                .map_err(|_| Status::InternalServerError)?;

            return Ok(Json(true));
        }
    }

    Ok(Json(false))
}