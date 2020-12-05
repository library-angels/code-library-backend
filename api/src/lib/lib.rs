use std::net::SocketAddr as Addr;

use warp::{filters::BoxedFilter, Reply, Server};

mod endpoints;
mod middleware;
mod response;
mod router;
mod rpc;

pub fn server(book_addr: Addr, identity_addr: Addr) -> Server<BoxedFilter<(impl Reply,)>> {
    warp::serve(crate::router::router(book_addr, identity_addr))
}
