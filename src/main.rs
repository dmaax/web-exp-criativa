#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, NamedFile};
use rocket::response::Redirect;
use std::path::Path;

#[get("/")]
fn root() -> Redirect {
    Redirect::to(uri!("/index"))
}

#[get("/<file>")]
async fn html_files(file: &str) -> Option<NamedFile> {
    let path = format!("static/{}.html", file);
    NamedFile::open(Path::new(&path)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root, html_files])
        .mount("/static", FileServer::from("static"))
}
