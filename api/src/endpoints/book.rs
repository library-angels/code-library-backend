use std::convert::Infallible;

use tarpc::context;
use warp::Reply;

use book::models::Error;
use helpers::ulid::Ulid;

use self::models::*;
use crate::middleware::session::Session;
use crate::response;

pub async fn get_book_by_id(_: Session, id: Ulid) -> Result<impl Reply, Infallible> {
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

pub async fn list_books(
    _: Session,
    query_params: models::QueryParams,
) -> Result<impl Reply, Infallible> {
    let QueryParams { page, page_size } = query_params;
    if let Ok(mut client) = crate::rpc::book_client().await {
        if let Ok(rpc_result) = client
            .list_books(
                context::current(),
                page.unwrap_or(1),
                page_size.unwrap_or(10),
            )
            .await
        {
            match rpc_result {
                Ok(res) => {
                    let (book_list, num_pages) = res;
                    return Ok(response::okay_with_json(&BookList {
                        book_list,
                        num_pages,
                    }));
                }
                Err(Error::InvalidData) => {
                    return Ok(response::bad_request("INVALID_QUERY_PARAMS"))
                }
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

mod models {
    use serde::{Deserialize, Serialize};

    use book::models::Book;

    #[derive(Debug, Deserialize)]
    pub struct QueryParams {
        pub page: Option<u32>,
        pub page_size: Option<u32>,
    }

    #[derive(Debug, Serialize)]
    pub struct BookList {
        pub book_list: Vec<Book>,
        pub num_pages: u32,
    }
}
