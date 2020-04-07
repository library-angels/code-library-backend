#[macro_use] 
extern crate diesel;
extern crate dotenv;
extern crate chrono;
use std::net::SocketAddr;
use dotenv::dotenv;
use warp::Filter;
use std::env;

mod query_models;
mod endpoints;
mod router;
mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "info");

    let address: SocketAddr = env::var("SERVER_ADDRESS_TEST")
        .expect("env var is not set").parse()
        .expect("can't parse env var");

    let db_state = db::db_connection::start_db().await;

    let routes = router::root()
        .or(router::identity())
        .or(router::book(db_state))
        .or(router::borrow())
        .or(router::notification());
        
    warp::serve(routes)
        .run(address)
        .await;
}
