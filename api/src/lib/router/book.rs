use crate::{endpoints::book::*, middleware::session::authorization};
use std::net::SocketAddr;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn book(book_addr: SocketAddr, identity_addr: SocketAddr) -> BoxedFilter<(impl Reply,)> {
    let book_filter = warp::any().map(move || book_addr);

    warp::path("book")
        .and(
            // GET /book
            warp::path::end()
                .and(authorization(identity_addr))
                .and(book_filter)
                .and(warp::get())
                .and(warp::query())
                .and_then(list_books)
                // GET /book/<book_id>
                .or(authorization(identity_addr)
                    .and(book_filter)
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and_then(get_book_by_id)),
        )
        .boxed()
}
