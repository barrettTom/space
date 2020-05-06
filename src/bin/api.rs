use actix_web::web::HttpResponse;
use actix_web::{get, put, web, App, HttpServer};
use actix_web_httpauth::extractors::basic::BasicAuth;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Deserialize;

use space::request::{Request, RequestData};

#[derive(Debug, Deserialize)]
struct Payload {
    module: String,
}

#[get("play")]
async fn play(
    auth: BasicAuth,
    payload: web::Json<Payload>,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> HttpResponse {
    let connection = pool.get().unwrap();

    let data = RequestData::Play {
        user: auth.user_id().to_string(),
        pass: auth.password().unwrap().to_string(),
        module: payload.module.clone(),
    };

    Request::new(data).get_http_response(&connection)
}

#[put("register")]
async fn register(
    auth: BasicAuth,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> HttpResponse {
    let connection = pool.get().unwrap();

    let data = RequestData::Register {
        user: auth.user_id().to_string(),
        pass: auth.password().unwrap().to_string(),
    };

    Request::new(data).get_http_response(&connection)
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
