use std::net::SocketAddr as Addr;

use warp::{filters::BoxedFilter, Reply, Server};

pub mod config;
mod endpoints;
mod middleware;
mod response;
mod router;

pub fn server(book_addr: Addr, identity_addr: Addr) -> Server<BoxedFilter<(impl Reply,)>> {
    warp::serve(crate::router::init_router(book_addr, identity_addr))
}
