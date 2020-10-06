use std::convert::Infallible;
use tarpc::context;
use warp::Reply;

use book_lib::models::Error;

use crate::middleware::session::Session;
use crate::response;

pub async fn get_book_by_id(_: Session, id: u32) -> Result<impl Reply, Infallible> {
    if let Ok(mut client) = crate::rpc::book_client().await {
        if let Ok(rpc_result) = client.get_book(context::current(), id).await {
            match rpc_result {
                Ok(book) => return Ok(response::okay_with_json(&book)),
                Err(Error::NotFound) => return Ok(response::not_found("BOOK_ID_NOT_FOUND")),
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}
