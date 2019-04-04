#[macro_use]
extern crate tera;

extern crate actix_web;
extern crate diesel;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tera::{Context, Tera};

use space::mass::MassEntry;
use space::math::get_db_url;
use space::schema::masses::dsl::masses as db_masses;

struct Pkg {
    pool: Pool<ConnectionManager<PgConnection>>,
    tera: Tera,
}

impl Pkg {
    pub fn new() -> Pkg {
        Pkg {
            pool: Pool::new(ConnectionManager::<PgConnection>::new(get_db_url())).unwrap(),
            tera: compile_templates!("templates/*"),
        }
    }
}

fn index(_: HttpRequest, data: web::Data<Pkg>) -> impl Responder {
    render(&data.tera, "index.html", &Context::new())
}

fn user(name: web::Path<String>, data: web::Data<Pkg>) -> impl Responder {
    let mut context = Context::new();
    context.insert("name", &name.into_inner());
    render(&data.tera, "user.html", &context)
}

fn leaderboards(_: HttpRequest, data: web::Data<Pkg>) -> impl Responder {
    let connection = data.pool.get().unwrap();
    let _mass_entries = db_masses
        .load::<MassEntry>(&connection)
        .expect("Cannot query, probably no migrations.");

    render(&data.tera, "leaderboards.html", &Context::new())
}

fn p404(_: HttpRequest, data: web::Data<Pkg>) -> impl Responder {
    render(&data.tera, "404.html", &Context::new())
}

fn render(tera: &Tera, html: &str, context: &Context) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(tera.render(html, context).unwrap())
}

fn main() {
    HttpServer::new(move || {
        App::new()
            .data(Pkg::new())
            .service(web::resource("/").to(index))
            .service(web::resource("/user/{name}").to(user))
            .service(web::resource("/leaderboards").to(leaderboards))
            .service(Files::new("/static", "static").show_files_listing())
            .default_resource(|request| request.route(web::get().to(p404)))
    })
    .bind("localhost:8000")
    .unwrap()
    .run()
    .unwrap()
}
