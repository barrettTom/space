use actix_web::{get, web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use serde::{Deserialize, Serialize};

use space::schema::users;

#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

#[derive(Debug)]
pub enum Request {
    Register { ship: String },
    Play { ship: String, module: String },
}

#[get("play/{ship}/{module}")]
async fn play(
    info: web::Path<(String, String)>,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    /*
    data.get_ref()
        .send(Request::Play {
            ship: info.0.clone(),
            module: info.1.clone(),
        })
        .unwrap();
    */
    "Good"
}

#[get("register/{ship}")]
async fn register(
    info: web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl Responder {
    /*
    data.get_ref()
        .send(Request::Register {
            ship: info.to_string(),
        })
        .unwrap();
    */
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
