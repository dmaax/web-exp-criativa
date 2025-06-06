
use rocket::serde::json::Json;
use serde::Serialize;
use dotenv::dotenv;
use std::env;

#[derive(Serialize)]
pub struct ChavePb{
    pub chavepb: String,

}

#[get("/pega-chave")]
pub fn pega_chave() -> Json<ChavePb>{
    dotenv().ok();

    let chave_carregada = env::var("PUBLIC_KEY").unwrap_or_else(|_| "chave_nao_encontrada".to_string());

    Json(ChavePb{ chavepb : chave_carregada,})

}