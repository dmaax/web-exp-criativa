#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, NamedFile};
use rocket::form::Form;
use std::path::Path;
mod cadastro;
 
// troquei pq a antiga forma (segundo a net da vida) fazia duas req agora ele ja envia direto o index :)
#[get("/")]
async fn root() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/<file>")]
async fn html_files(file: &str) -> Option<NamedFile> {
    let path = format!("static/{}.html", file);
    NamedFile::open(Path::new(&path)).await.ok()
}


#[post("/criar-conta", data = "<cadastro>")]
async fn cadastrar(cadastro: Form<cadastro::Cadastro>) -> String {
    cadastro::cadastrar(cadastro).await // espera 

}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root, html_files, cadastrar])
        .mount("/static", FileServer::from("static"))
}
