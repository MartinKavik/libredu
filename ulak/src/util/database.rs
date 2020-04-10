//  database.rs
//
//  Handles setting up database routines, state, and such
//  to work within actix-web.
//
//  @author Ryan McGrath <ryan@rymc.io>
//  @created 06/16/2018
use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager};
use dotenv::dotenv;
//use std::thread;
pub fn pool() -> r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pol = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();
    let con = pol.get().unwrap();
    con
}

pub fn pool2() -> r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pol = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("hata");
    pol
}