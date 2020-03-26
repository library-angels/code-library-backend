use warp::Filter;
use log::error;
use std::process;

mod router;
mod endpoints;
mod config;


#[tokio::main]
async fn main() {
    env_logger::init();

    let config = match config::Config::new() {
        Ok(val) => val,
        Err(config::Error::HttpHostInvalidValue) => {
            error!("Terminating, due to invalid 'HTTP_HOST' environment variable");
            process::exit(1);
        }
        Err(config::Error::HttpPortInvalidValue) => {
            error!("Terminating, due to invalid 'HTTP_PORT' environment variable");
            process::exit(1);
        }
    };
    config.log_info_configuration();

    let routes = router::root()
        .or(router::identity())
        .or(router::book())
        .or(router::borrow())
        .or(router::notification());

    warp::serve(routes)
        .try_bind(config.http_socket())
        .await;
}
