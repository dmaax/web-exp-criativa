use rocket::{get, http::Status, serde::json::Json};
use rocket::{post, serde::json::Json as RocketJson};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::{contas, extratos};
use crate::login_db::conectar_escritor_leitor;
use crate::SessaoUsuario;
use crate::schema::usuarios;

#[derive(Serialize)]
pub struct DadosConta {
    pub saldo_conta: String,
    pub saldo_poupanca: String,
    pub transacoes: Vec<String>,
}

#[derive(Deserialize)]
pub struct DepositoRequest {
    pub valor: f64,
}

#[derive(Deserialize)]
pub struct PagamentoRequest {
    pub valor: f64,
}

// o certo é tirara esses poupanca, mas fica ai um pouco de castigo kkk
#[get("/dados-conta")]
pub async fn dados_conta(sessao: SessaoUsuario) -> Result<Json<DadosConta>, Status> {
    let mut conn = conectar_escritor_leitor();

    // Busca o id da conta do usuário autenticado
    let conta_id_result = contas::dsl::contas
        .inner_join(usuarios::dsl::usuarios.on(contas::dsl::usuario_id.eq(usuarios::dsl::id)))
        .filter(usuarios::dsl::id.eq(sessao.0))
        .select(contas::dsl::id)
        .first::<i32>(&mut conn)
        .optional();

    let conta_id = match conta_id_result {
        Ok(Some(id)) => id,
        _ => return Err(Status::NotFound),
    };

    // Buscar saldo da conta
    let saldo_conta_result: Result<String, _> = contas::dsl::contas
        .filter(contas::dsl::id.eq(conta_id))
        .select(contas::dsl::saldo)
        .first(&mut conn);

    let saldo_conta = saldo_conta_result.unwrap_or_else(|_| "0.00".to_string());

    // Exemplo: saldo da poupança simulado (em produção, seria de uma tabela real)
    let saldo_poupanca = "50.00".to_string();

    // Buscar transações do extrato
    let extratos_result: Result<Vec<(String, String)>, _> = extratos::dsl::extratos
        .filter(extratos::dsl::conta_id.eq(conta_id))
        .select((extratos::dsl::nome_compra, extratos::dsl::valor))
        .load(&mut conn);

    let transacoes = extratos_result
        .unwrap_or_default()
        .into_iter()
        .map(|(nome, valor)| format!("{} - R$ {}", nome, valor))
        .collect();

    Ok(Json(DadosConta {
        saldo_conta: format!("R$ {}", saldo_conta),
        saldo_poupanca: format!("R$ {}", saldo_poupanca),
        transacoes,
    }))
}

#[post("/depositar", format = "json", data = "<deposito>")]
pub async fn depositar(sessao: SessaoUsuario, deposito: RocketJson<DepositoRequest>) -> Result<RocketJson<DadosConta>, Status> {
    let mut conn = conectar_escritor_leitor();

    let conta_id = contas::dsl::contas
        .inner_join(usuarios::dsl::usuarios.on(contas::dsl::usuario_id.eq(usuarios::dsl::id)))
        .filter(usuarios::dsl::id.eq(sessao.0))
        .select(contas::dsl::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    // Buscar saldo atual como String
    let saldo_atual_result: Result<String, _> = contas::dsl::contas
        .filter(contas::dsl::id.eq(conta_id))
        .select(contas::dsl::saldo)
        .first(&mut conn);

    // Converter saldo para f64, tratar caso de erro
    let saldo_atual_f64 = match saldo_atual_result {
        Ok(ref s) => s.replace(",", ".").parse::<f64>().unwrap_or(0.0),
        Err(_) => 0.0,
    };

    let novo_saldo_f64 = saldo_atual_f64 + deposito.valor;
    let novo_saldo_str = format!("{:.2}", novo_saldo_f64);

    // Atualizar saldo no banco (como string)
    let update_result = diesel::update(contas::dsl::contas.filter(contas::dsl::id.eq(conta_id)))
        .set(contas::dsl::saldo.eq(&novo_saldo_str))
        .execute(&mut conn);

    if let Err(e) = update_result {
        eprintln!("Erro ao atualizar saldo: {:?}", e);
        return Err(Status::InternalServerError);
    }

    // Adicionar transação no extrato
    let insert_result = diesel::insert_into(extratos::dsl::extratos)
        .values((
            extratos::dsl::conta_id.eq(conta_id),
            extratos::dsl::nome_compra.eq("Depósito"),
            extratos::dsl::valor.eq(format!("{:.2}", deposito.valor)),
        ))
        .execute(&mut conn);

    if let Err(e) = insert_result {
        eprintln!("Erro ao inserir extrato: {:?}", e);
        // Não retorna erro para não travar o fluxo do usuário
    }

    // Buscar extratos atualizados
    let extratos_result: Result<Vec<(String, String)>, _> = extratos::dsl::extratos
        .filter(extratos::dsl::conta_id.eq(conta_id))
        .select((extratos::dsl::nome_compra, extratos::dsl::valor))
        .order(extratos::dsl::id.desc())
        .limit(10)
        .load(&mut conn);

    let transacoes = extratos_result
        .unwrap_or_default()
        .into_iter()
        .map(|(nome, valor)| format!("{} - R$ {}", nome, valor))
        .collect();

    Ok(RocketJson(DadosConta {
        saldo_conta: format!("R$ {}", novo_saldo_str),
        saldo_poupanca: format!("R$ {}", "50.00"),
        transacoes,
    }))
}

#[post("/pagar-divida", format = "json", data = "<pagamento>")]
pub async fn pagar_divida(sessao: SessaoUsuario, pagamento: RocketJson<PagamentoRequest>) -> Result<RocketJson<DadosConta>, Status> {
    let mut conn = conectar_escritor_leitor();

    let conta_id = contas::dsl::contas
        .inner_join(usuarios::dsl::usuarios.on(contas::dsl::usuario_id.eq(usuarios::dsl::id)))
        .filter(usuarios::dsl::id.eq(sessao.0))
        .select(contas::dsl::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    // Buscar saldo atual
    let saldo_atual_result: Result<String, _> = contas::dsl::contas
        .filter(contas::dsl::id.eq(conta_id))
        .select(contas::dsl::saldo)
        .first(&mut conn);

    let saldo_atual_f64 = match saldo_atual_result {
        Ok(ref s) => s.replace(",", ".").parse::<f64>().unwrap_or(0.0),
        Err(_) => 0.0,
    };

    if pagamento.valor > saldo_atual_f64 {
        return Err(Status::BadRequest);
    }

    let novo_saldo_f64 = saldo_atual_f64 - pagamento.valor;
    let novo_saldo_str = format!("{:.2}", novo_saldo_f64);

    // Atualizar saldo da conta
    let update_result = diesel::update(contas::dsl::contas.filter(contas::dsl::id.eq(conta_id)))
        .set(contas::dsl::saldo.eq(&novo_saldo_str))
        .execute(&mut conn);

    if let Err(e) = update_result {
        eprintln!("Erro ao atualizar saldo: {:?}", e);
        return Err(Status::InternalServerError);
    }

    // Diminuir saldo_usado do cartão (primeiro cartão encontrado da conta)
    use crate::schema::cartoes;
    if let Ok((cartao_id, _conta_id, _numero, _data, _codigo, _limite, saldo_usado)) = cartoes::dsl::cartoes
        .filter(cartoes::dsl::conta_id.eq(conta_id))
        .select((
            cartoes::dsl::id,
            cartoes::dsl::conta_id,
            cartoes::dsl::numero_cartao,
            cartoes::dsl::data_cartao,
            cartoes::dsl::codigo_cartao,
            cartoes::dsl::saldo_disponivel,
            cartoes::dsl::saldo_usado,
        ))
        .first::<(i32, i32, String, String, String, String, String)>(&mut conn)
    {
        let saldo_usado_f = saldo_usado.replace(",", ".").parse::<f64>().unwrap_or(0.0);
        let novo_usado = (saldo_usado_f - pagamento.valor).max(0.0);
        let _ = diesel::update(cartoes::dsl::cartoes.filter(cartoes::dsl::id.eq(cartao_id)))
            .set(cartoes::dsl::saldo_usado.eq(format!("{:.2}", novo_usado)))
            .execute(&mut conn);
    }

    // Adicionar transação no extrato
    let insert_result = diesel::insert_into(extratos::dsl::extratos)
        .values((
            extratos::dsl::conta_id.eq(conta_id),
            extratos::dsl::nome_compra.eq("Pagamento de Dívida"),
            extratos::dsl::valor.eq(format!("-{:.2}", pagamento.valor)),
        ))
        .execute(&mut conn);

    if let Err(e) = insert_result {
        eprintln!("Erro ao inserir extrato: {:?}", e);
    }

    // Buscar extratos atualizados
    let extratos_result: Result<Vec<(String, String)>, _> = extratos::dsl::extratos
        .filter(extratos::dsl::conta_id.eq(conta_id))
        .select((extratos::dsl::nome_compra, extratos::dsl::valor))
        .order(extratos::dsl::id.desc())
        .limit(10)
        .load(&mut conn);

    let transacoes = extratos_result
        .unwrap_or_default()
        .into_iter()
        .map(|(nome, valor)| format!("{} - R$ {}", nome, valor))
        .collect();

    Ok(RocketJson(DadosConta {
        saldo_conta: format!("R$ {}", novo_saldo_str),
        saldo_poupanca: format!("R$ {}", "50.00"),
        transacoes,
    }))
}
