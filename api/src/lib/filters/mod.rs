pub mod authorization;
pub mod pagination;
pub mod rejection;

use std::net::SocketAddr;
use warp::{filters::BoxedFilter, Filter};

pub fn book_service(book_addr: SocketAddr) -> BoxedFilter<(SocketAddr,)> {
    warp::any().map(move || book_addr).boxed()
}

pub fn identity_filter(identity_addr: SocketAddr) -> BoxedFilter<(SocketAddr,)> {
    warp::any().map(move || identity_addr).boxed()
}
