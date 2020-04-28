#[macro_use]
extern crate diesel;
use std::net::SocketAddr;
use warp::Filter;

mod config;
mod db;
mod endpoints;
mod query_models;
mod router;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = config::initialize_config().unwrap();
    let db_state = db::db_connection::start_db(config.database_url.clone()).await;
    let routes = router::root()
        .or(router::identity())
        .or(router::book(db_state))
        .or(router::borrow())
        .or(router::notification());

    warp::serve(routes)
        .try_bind(SocketAddr::new(config.http_host, config.http_port))
        .await;
}
