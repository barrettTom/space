use actix_rt::System;
use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::mpsc;

#[derive(Debug)]
pub enum Request {
    Register { ship: String },
    Play { ship: String, module: String },
}

#[get("play/{ship}/{module}")]
async fn main(
    info: web::Path<(String, String)>,
    data: web::Data<mpsc::Sender<Request>>,
) -> impl Responder {
    data.get_ref()
        .send(Request::Play {
            ship: info.0.clone(),
            module: info.1.clone(),
        })
        .unwrap();

    "Good"
}

#[get("register/{ship}")]
async fn register(
    info: web::Path<String>,
    data: web::Data<mpsc::Sender<Request>>,
) -> impl Responder {
    data.get_ref()
        .send(Request::Register {
            ship: info.to_string(),
        })
        .unwrap();

    "Good"
}

pub fn run(tx: mpsc::Sender<Request>) -> std::io::Result<()> {
    let mut sys = System::new("runtime");

    sys.block_on(
        HttpServer::new(move || App::new().data(tx.clone()).service(main).service(register))
            .bind("127.0.0.1:8000")?
            .run(),
    )
}
