use crate::{
    endpoints::book::*,
    filters::{authorization::authorization, book_filter},
};
use std::net::SocketAddr;
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn book(book_addr: SocketAddr, identity_addr: SocketAddr) -> BoxedFilter<(impl Reply,)> {
    warp::path("book")
        .and(
            // GET /book
            warp::path::end()
                .and(authorization(identity_addr))
                .and(book_filter(book_addr))
                .and(warp::get())
                .and_then(list_books)
                // GET /book/<book_id>
                .or(authorization(identity_addr)
                    .and(book_filter(book_addr))
                    .and(warp::path::param::<Uuid>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and_then(get_book_by_id)),
        )
        .boxed()
}
