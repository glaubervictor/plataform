use actix_web::{middleware, web::Data, App, HttpServer};
use controllers::app_configure;
use env_logger::Env;
use std::io;

use database::get_pool;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

mod config;
mod controllers;
mod database;
mod error;
mod models;
mod repositories;
mod schema;
mod validate;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_module_path(false)
        .init();
    
    info!("🚀 starting up");

    let pool = get_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(app_configure)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
