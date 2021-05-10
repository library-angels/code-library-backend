use std::{convert::Infallible, net::SocketAddr};

use uuid::Uuid;
use warp::Reply;

use crate::filters::authorization::Session;

pub async fn get_book_by_id(
    _: Session,
    _addr: SocketAddr,
    _id: Uuid,
) -> Result<impl Reply, Infallible> {
    Ok(warp::reply())
}

pub async fn list_books(_: Session, _book_addr: SocketAddr) -> Result<impl Reply, Infallible> {
    Ok(warp::reply())
}
