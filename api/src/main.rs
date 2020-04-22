#[macro_use]
extern crate diesel;
use dotenv::dotenv;
use envconfig::Envconfig;
use log::error;
use std::net::SocketAddr;
use std::process;
use warp::Filter;

mod config;
mod db;
mod endpoints;
mod query_models;
mod router;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    let config = match config::Config::init() {
        Ok(val) => val,
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    };

    let db_state = db::db_connection::start_db(config.database_url).await;
    let routes = router::root()
        .or(router::identity())
        .or(router::book(db_state))
        .or(router::borrow())
        .or(router::notification());

    warp::serve(routes)
        .try_bind(SocketAddr::new(config.http_host, config.http_port))
        .await;
}
