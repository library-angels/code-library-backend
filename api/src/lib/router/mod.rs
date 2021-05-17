mod book;
mod identity;
mod root;

use std::net::SocketAddr;

use crate::middleware::rejection::handle_rejection;

use warp::{filters::BoxedFilter, Filter, Reply};

pub fn init_router(book_addr: SocketAddr, identity_addr: SocketAddr) -> BoxedFilter<(impl Reply,)> {
    root::root()
        .or(identity::identity(identity_addr))
        .or(book::book(book_addr, identity_addr))
        .recover(handle_rejection)
        .boxed()
}
