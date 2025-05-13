use rocket::{get, post, http::Status, serde::json::Json};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::{cartoes, extratos};
use crate::login_db::conectar_escritor_leitor;

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

#[derive(Serialize)]
pub struct ListaCartoes {
    pub cartoes: Vec<CartaoInfo>,
}

#[derive(Deserialize)]
pub struct CompraRequest {
    pub cartao_id: i32,
    pub nome_compra: String,
    pub valor: f64,
}

#[get("/cartoes")]
pub async fn listar_cartoes() -> Result<Json<ListaCartoes>, Status> {
    let mut conn = conectar_escritor_leitor();
    let conta_id_simulada = 1;

    let result = cartoes::dsl::cartoes
        .filter(cartoes::dsl::conta_id.eq(conta_id_simulada))
        .load::<(i32, i32, String, String, String, String, String)>(&mut conn);

    let cartoes_vec = match result {
        Ok(rows) => rows.into_iter().map(|(id, _conta_id, numero, data_cartao, codigo_cartao, saldo_disp, saldo_usado)| {
            CartaoInfo {
                id,
                label: format!("Cartão {}", &numero[numero.len().saturating_sub(4)..]),
                numero,
                data_cartao,
                codigo_cartao,
                limite: format!("R$ {}", saldo_disp),
                usado: format!("R$ {}", saldo_usado),
            }
        }).collect(),
        Err(_) => vec![],
    };

    Ok(Json(ListaCartoes { cartoes: cartoes_vec }))
}

#[post("/compra", format = "json", data = "<compra>")]
pub async fn registrar_compra(compra: Json<CompraRequest>) -> Result<Status, Status> {
    let mut conn = conectar_escritor_leitor();

    // Buscar saldo disponível e usado
    let cartao = cartoes::dsl::cartoes
        .filter(cartoes::dsl::id.eq(compra.cartao_id))
        .first::<(i32, i32, String, String, String, String, String)>(&mut conn)
        .ok();  

    if let Some((_id, conta_id, _numero, _data, _codigo, saldo_disp, saldo_usado)) = cartao {
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
