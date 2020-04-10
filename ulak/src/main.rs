#![allow(proc_macro_derive_resolution_fallback)]
use actix_files as fs;
extern crate actix_rt;
extern crate actix_web;
extern crate serde;
extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate uuid;
extern crate rand;
extern crate hashbrown;
//extern crate models;
//extern crate validator;
extern crate env_logger;
extern crate bcrypt;

//use chrono::Duration;

use actix_redis::{RedisActor, RedisSession};
//use actix_identity::{CookieIdentityPolicy, IdentityService};
#[warn(unused_mut)]
#[macro_use] extern crate diesel;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate tera;
pub mod models;
pub mod schema;
pub mod forms;
pub mod views;
pub mod school;
pub mod teachers;
pub mod util;
pub mod classes;
pub mod generate;
pub mod admin;
pub mod subjects;
pub mod timetable;
pub mod request;
pub mod websocket;
pub mod chat_server;

use actix_web::*;
use actix_web::cookie::*;
use std::{env, io};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix::Actor;
use std::path::Path;
//use actix_session::{CookieSession, Session};
//use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Result};


#[actix_rt::main]
async fn main() -> io::Result<()>{
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    //let _sys = actix::System::new("Ulak");
    //let addr = addr.clone();
    let db = util::database::pool2();
    let server = chat_server::ChatServer::default().start();
    use chrono::{Duration};
    HttpServer::new(move || {
        let path = Path::new("./client/pkg");
        let redis_addr = RedisActor::start("127.0.0.1:6379");
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
        let mut secure = true;
        let domain: String =
            std::env::var("DOMAIN").unwrap_or_else(move |_| "127.0.0.1".to_string());
        if domain == "127.0.0.1".to_string() {
            secure = false;
        };
        App::new()
            .data(tera)
            .data(server.clone())
            .data(db.clone())
            .data(redis_addr)
            .wrap(middleware::Logger::default())
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32])
                .ttl(65535)
                .cookie_name("Ulak")
                .cookie_same_site(SameSite::None)
                .cookie_domain(&domain.to_string())
                .cookie_max_age(Duration::seconds(65535))
                .cookie_secure(secure))
            .wrap(IdentityService::new(
                 CookieIdentityPolicy::new(&[0; 32])
                     .domain(domain)
                     .name("Ulak")
                     .path("/")
                     .max_age(65535)
                     .secure(secure),))
            //.wrap(CookieSession::signed(&[0; 32]).secure(secure)
            //    .max_age(60*60*24*360))
            //.service(fs::Files::new("/", "static/ws").index_file("index.html"))
            .service(// static files
            fs::Files::new("/static",path)
            )
            .service(web::resource("/").to(views::index))
            //.service(web::resource("/admin").to(admin::index))
            .service(web::resource("/login").to(views::login))
            .service(web::resource("/signin").to(views::signin))
            .service(web::resource("/logout").to(views::logout))
            .service(web::resource("/cities/{id}").to(views::cities))
            .service(web::scope("/admin")
                .service(web::resource("")
                    .to(admin::index))
                .service(web::resource("/town")
                    .to(admin::town))
                .service(web::resource("/subject")
                    .to(admin::subject)))
            .service(web::scope("/school")
                .service(web::resource("/add")
                    .to(school::add))
                .service(web::scope("/{code}")
                    .service(web::resource("")
                        .to(school::detail))
                    .service(web::resource("/teachers")
                        .to(school::teachers))
                    .service(web::resource("/classes")
                        .to(school::classes))
                    .service(web::scope("/timetable")
                        .service(web::resource("")
                            .to(timetable::timetable))
                        .service(web::resource("/classes")
                            .to(timetable::class_timetable))
                        .service(web::resource("/generate")
                            .to(generate::generate))
                        .service(web::resource("/teachers")
                            .to(timetable::teacher_timetable)))
                    .service(web::resource("/subjects")
                        .to(school::subjects))
                    .service(web::resource("/activities")
                        .to(school::activities))
                    .service(web::resource("/activities/{id}")
                        .to(school::activities_detail))
                    .service(web::scope("/teachers")
                        .service(web::resource("{id}")
                            .to(teachers::detail))
                        .service(web::resource("{id}/activities")
                            .to(teachers::activities))
                        .service(web::resource("{id}/limitation")
                            .to(teachers::limitations))
                        .service(web::resource("{id}/timetable")
                            .to(teachers::timetable))
                        )
                    .service(web::scope("/classes")
                        .service(web::resource("{id}")
                            .to(classes::detail))
                        .service(web::resource("{id}/activities")
                            .to(classes::activities))
                        .service(web::resource("{id}/limitation")
                            .to(classes::limitations))
                        .service(web::resource("{id}/timetable")
                            .to(classes::timetable))
                        )
                    )
            )
            .service(web::resource("/subjects")
                .to(subjects::index))
            .service((web::resource("/ws/{code}"))
                .to(websocket::chat_route))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
