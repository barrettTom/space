use actix_web::{get, web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use space::request::{Request, RequestData};

#[get("play")]
async fn play(
    info: web::Path<(String, String)>,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    let connection = pool.get().unwrap();

    let data = RequestData::Play {
        ship: info.0.to_string(),
        module: info.1.to_string(),
    };

    let request = Request::new(data);

    web::block(move || request.get_response(&connection))
        .await
        .unwrap()
        .to_string()
}

#[get("register")]
async fn register(
    info: web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    let connection = pool.get().unwrap();

    let data = RequestData::Register {
        ship: info.to_string(),
    };

    let request = Request::new(data);

    web::block(move || request.get_response(&connection))
        .await
        .unwrap()
        .to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<SqliteConnection>::new("space.db");
    let pool = Pool::builder().build(manager).unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(play)
            .service(register)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
