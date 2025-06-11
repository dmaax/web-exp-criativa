
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

    let chave_carregada = env::var("CHAVE_PUBLICA").unwrap_or_else(|_| "chave_nao_encontrada".to_string());

    Json(ChavePb{ chavepb : chave_carregada,})

}

//CHAVE_PUBLICA="-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAlE6F/CmyUR6JXafRZ+G8\nRz1P0Ij2I8pokVvEn85y/r1z5SFsTQPtlqA202sm0VkwHgMi+cK2CoD+E7YyfTWU\nWxtFj+WRPiyq/2NYfPGLxGQMOUrlxZWSMWzThdw6MM901YA2wzirMQaVu/mWX17m\n3tweQi2AQgMtRaT8WOzmBjNd3iaA8UHifEBC98yzEt5ld0pQi6YpqluQx2aK5L3C\nFrs5j/zKZoZakxU2RDlvZKfwmxm6VUpHl32Ac0sGTOsWZe1V0QYKz7+0ckE7nu1u\nZul+VrBmdRBTviNnOaGXLei3f+hN4+AYtuBdu4TOCOQ904nxGpsiXJVhPDk5tHVx\nNQIDAQAB\n-----END PUBLIC KEY-----"
