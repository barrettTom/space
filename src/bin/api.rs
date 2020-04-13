#[macro_use]
extern crate diesel;

use actix_web::{get, web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Serialize;
use std::time::SystemTime;
use uuid::Uuid;

use space::schema::requests;

#[derive(Debug, Serialize, Queryable, Insertable)]
pub struct Request {
    id: String,
    data: String,
    time: String,
    received: bool,
}

impl Request {
    pub fn new(data: RequestData) -> Request {
        Request {
            id: Uuid::new_v4().to_string(),
            data: serde_json::to_string(&data).unwrap(),
            time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            received: false,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum RequestData {
    Play { ship: String, module: String },
    Register { ship: String },
}

#[get("play/{ship}/{module}")]
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

    web::block(move || {
        diesel::insert_into(requests::dsl::requests)
            .values(&request)
            .execute(&connection)
    })
    .await
    .unwrap();

    "Good"
}

#[get("register/{ship}")]
async fn register(
    info: web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    let connection = pool.get().unwrap();

    let data = RequestData::Register {
        ship: info.to_string(),
    };

    let request = Request::new(data);

    web::block(move || {
        diesel::insert_into(requests::dsl::requests)
            .values(&request)
            .execute(&connection)
    })
    .await
    .unwrap();

    "Good"
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
