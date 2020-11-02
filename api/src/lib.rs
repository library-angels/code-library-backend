use once_cell::sync::OnceCell;
use warp::{filters::BoxedFilter, Reply};

mod config;
mod endpoints;
mod middleware;
mod router;
mod rpc;

pub use crate::config::Configuration;

pub static CONFIGURATION: OnceCell<Configuration> = OnceCell::new();

pub fn server() -> warp::Server<BoxedFilter<(impl Reply,)>> {
    warp::serve(crate::router::router())
}
