use actix_rt::System;
use actix_web::{dev::Server, get, web, App, HttpServer, Responder};
use std::sync::mpsc;

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

pub fn run(tx: mpsc::Sender<Server>) -> std::io::Result<()> {
    let mut sys = System::new("runtime");

    sys.block_on(
        HttpServer::new(move || {
            App::new()
                .data(tx.clone())
                .service(web::resource("/").to(|| async { "hallo" }))
                .service(web::resource("/user/").to(|| async { "bye" }))
                .service(index)
        })
        .bind("127.0.0.1:8000")?
        .run(),
    )
}
