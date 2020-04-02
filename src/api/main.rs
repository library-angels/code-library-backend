use warp::Filter;
use envconfig::Envconfig;
use std::process;
use log::error;
use std::net::SocketAddr;
use dotenv::dotenv;

mod config;
mod router;
mod endpoints;


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

    let routes = router::root()
        .or(router::identity())
        .or(router::book())
        .or(router::borrow())
        .or(router::notification());

    warp::serve(routes)
        .try_bind(SocketAddr::new(config.http_host, config.http_port))
        .await;
}
