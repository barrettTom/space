#[macro_use]
extern crate tera;

extern crate actix_web;
extern crate diesel;
extern crate ring;

use actix_files::Files;
use actix_web::middleware::identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tera::{Context, Tera};

use space::db::{get_db_url, Login, MassEntry, Registration};
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

fn index(id: Identity, data: web::Data<Pkg>) -> impl Responder {
    render(&data, id, "index.html", &mut Context::new())
}

fn user(id: Identity, name: web::Path<String>, data: web::Data<Pkg>) -> impl Responder {
    let mut context = Context::new();
    context.insert("name", &name.into_inner());
    render(&data, id, "user.html", &mut context)
}

fn leaderboards(id: Identity, data: web::Data<Pkg>) -> impl Responder {
    let connection = data.pool.get().unwrap();
    let _mass_entries = db_masses
        .load::<MassEntry>(&connection)
        .expect("Cannot query, probably no migrations.");

    render(&data, id, "leaderboards.html", &mut Context::new())
}

fn docs(id: Identity, data: web::Data<Pkg>) -> impl Responder {
    render(&data, id, "docs.html", &mut Context::new())
}

fn register(id: Identity, data: web::Data<Pkg>) -> impl Responder {
    let mut context = Context::new();
    context.insert("error", &false);
    render(&data, id, "register.html", &mut context)
}

fn post_register(
    id: Identity,
    form: web::Form<Registration>,
    data: web::Data<Pkg>,
) -> impl Responder {
    match form.insert_into(data.pool.get().unwrap()) {
        Ok(_) => {
            id.remember(form.name.to_owned());
            render(&data, id, "index.html", &mut Context::new())
        }
        Err(error) => {
            let mut context = Context::new();
            context.insert("error", &error);
            render(&data, id, "register.html", &mut context)
        }
    }
}

fn login(id: Identity, data: web::Data<Pkg>) -> impl Responder {
    render(&data, id, "login.html", &mut Context::new())
}

fn post_login(id: Identity, form: web::Form<Login>, data: web::Data<Pkg>) -> impl Responder {
    match form.verify(data.pool.get().unwrap()) {
        Ok(_) => {
            id.remember(form.name.to_owned());
            render(&data, id, "index.html", &mut Context::new())
        }
        Err(error) => {
            let mut context = Context::new();
            context.insert("error", &error);
            render(&data, id, "login.html", &mut context)
        }
    }
}

fn logout(id: Identity, data: web::Data<Pkg>) -> impl Responder {
    id.forget();
    render(&data, id, "index.html", &mut Context::new())
}

fn p404(id: Identity, data: web::Data<Pkg>) -> impl Responder {
    render(&data, id, "404.html", &mut Context::new())
}

fn render(
    data: &web::Data<Pkg>,
    id: Identity,
    html: &str,
    context: &mut Context,
) -> impl Responder {
    match &id.identity() {
        Some(identity) => context.insert("user", &identity),
        None => context.insert("user", &false),
    }
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(data.tera.render(html, context).unwrap())
}

fn main() {
    HttpServer::new(move || {
        App::new()
            .data(Pkg::new())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32]).secure(false),
            ))
            .service(web::resource("/").to(index))
            .service(web::resource("/user/{name}").to(user))
            .service(web::resource("/leaderboards").to(leaderboards))
            .service(web::resource("/docs").to(docs))
            .service(web::resource("/login").to(login))
            .service(web::resource("/post_login").to(post_login))
            .service(web::resource("/register").to(register))
            .service(web::resource("/post_register").to(post_register))
            .service(web::resource("/logout").to(logout))
            .service(Files::new("/static", "static").show_files_listing())
            .default_resource(|request| request.route(web::get().to(p404)))
    })
    .bind("localhost:8000")
    .unwrap()
    .run()
    .unwrap()
}
