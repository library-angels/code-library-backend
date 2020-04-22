use warp::Filter;
use envconfig::Envconfig;
use std::process;
use log::error;
use std::net::SocketAddr;
use dotenv::dotenv;
use std::sync::Arc;

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


    let config = Arc::new(Box::new(config));
    let routes = router::root()
        .or(router::identity(config.clone()))
        .or(router::book())
        .or(router::borrow())
        .or(router::notification())
        .with(warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE"])
    );

    warp::serve(routes)
        .try_bind(SocketAddr::new(config.http_host, config.http_port))
        .await;
}
