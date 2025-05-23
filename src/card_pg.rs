use rocket::{get, post, http::Status, serde::json::Json};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::{cartoes, extratos};
use crate::login_db::conectar_escritor_leitor;
use crate::SessaoUsuario;
use crate::schema::usuarios;
use crate::schema::contas;

#[derive(Serialize)]
pub struct CartaoInfo {
    pub id: i32,
    pub label: String,
    pub numero: String,
    pub data_cartao: String,
    pub codigo_cartao: String,
    pub limite: String,
    pub usado: String,
}

#[derive(Deserialize)]
pub struct CompraRequest {
    pub cartao_id: i32,
    pub nome_compra: String,
    pub valor: f64,
}

#[get("/cartoes")]
pub async fn listar_cartoes(sessao: SessaoUsuario) -> Result<Json<CartaoInfo>, Status> {
    let mut conn = conectar_escritor_leitor();

    // Busca o id da conta do usuário autenticado
    let conta_id = contas::dsl::contas
        .inner_join(usuarios::dsl::usuarios.on(contas::dsl::usuario_id.eq(usuarios::dsl::id)))
        .filter(usuarios::dsl::id.eq(sessao.0))
        .select(contas::dsl::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    let result = cartoes::dsl::cartoes
        .filter(cartoes::dsl::conta_id.eq(conta_id))
        .first::<(i32, i32, String, String, String, String, String)>(&mut conn);

    match result {
        Ok((id, _conta_id, numero, data_cartao, codigo_cartao, saldo_disp, saldo_usado)) => {
            let cartao = CartaoInfo {
                id,
                label: format!("Cartão {}", &numero[numero.len().saturating_sub(4)..]),
                numero,
                data_cartao,
                codigo_cartao,
                limite: format!("R$ {}", saldo_disp),
                usado: format!("R$ {}", saldo_usado),
            };
            Ok(Json(cartao))
        }
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/compra", format = "json", data = "<compra>")]
pub async fn registrar_compra(sessao: SessaoUsuario, compra: Json<CompraRequest>) -> Result<Status, Status> {
    let mut conn = conectar_escritor_leitor();

    // Busca o id da conta do usuário autenticado
    let conta_id = contas::dsl::contas
        .inner_join(usuarios::dsl::usuarios.on(contas::dsl::usuario_id.eq(usuarios::dsl::id)))
        .filter(usuarios::dsl::id.eq(sessao.0))
        .select(contas::dsl::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    // Buscar saldo disponível e usado do cartão do usuário
    let cartao = cartoes::dsl::cartoes
        .filter(cartoes::dsl::id.eq(compra.cartao_id))
        .filter(cartoes::dsl::conta_id.eq(conta_id))
        .first::<(i32, i32, String, String, String, String, String)>(&mut conn)
        .ok();

    if let Some((_id, _conta_id, _numero, _data, _codigo, saldo_disp, saldo_usado)) = cartao {
        let saldo_disp_f = saldo_disp.replace(",", ".").parse::<f64>().unwrap_or(0.0);
        let saldo_usado_f = saldo_usado.replace(",", ".").parse::<f64>().unwrap_or(0.0);

        if compra.valor > (saldo_disp_f - saldo_usado_f) {
            return Err(Status::BadRequest);
        }

        let novo_usado = saldo_usado_f + compra.valor;

        // Atualiza saldo usado
        let _ = diesel::update(cartoes::dsl::cartoes.filter(cartoes::dsl::id.eq(compra.cartao_id)))
            .set(cartoes::dsl::saldo_usado.eq(format!("{:.2}", novo_usado)))
            .execute(&mut conn);

        // Adiciona no extrato
        let _ = diesel::insert_into(extratos::dsl::extratos)
            .values((
                extratos::dsl::conta_id.eq(conta_id),
                extratos::dsl::nome_compra.eq(&compra.nome_compra),
                extratos::dsl::valor.eq(format!("{:.2}", compra.valor)),
            ))
            .execute(&mut conn);

        Ok(Status::Ok)
    } else {
        Err(Status::NotFound)
    }
}
