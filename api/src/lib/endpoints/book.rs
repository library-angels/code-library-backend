use std::{convert::Infallible, net::SocketAddr};

use tarpc::context;
use uuid::Uuid;
use warp::Reply;

use book::init_rpc_client;
use helpers::{filters, rpc::Error};

use crate::response;

pub async fn get_books(
    addr: SocketAddr,
    page: filters::Page,
    book: filters::Book,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(books) = client.get_books(context::current(), page, book).await {
            return Ok(response::okay_with_json(&books));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_authors(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(authors) = client.get_authors(context::current(), page).await {
            return Ok(response::okay_with_json(&authors));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_categories(
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(categories) = client.get_categories(context::current(), page).await {
            return Ok(response::okay_with_json(&categories));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_copies(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(copies) = client.get_copies(context::current(), page).await {
            return Ok(response::okay_with_json(&copies));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_editors(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(editors) = client.get_editors(context::current(), page).await {
            return Ok(response::okay_with_json(&editors));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_languages(
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(languages) = client.get_languages(context::current(), page).await {
            return Ok(response::okay_with_json(&languages));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_publishers(
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(publishers) = client.get_publishers(context::current(), page).await {
            return Ok(response::okay_with_json(&publishers));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_series(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(series) = client.get_series(context::current(), page).await {
            return Ok(response::okay_with_json(&series));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_subject_areas(
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(subject_areas) = client.get_subject_areas(context::current(), page).await {
            return Ok(response::okay_with_json(&subject_areas));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_tags(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(tags) = client.get_tags(context::current(), page).await {
            return Ok(response::okay_with_json(&tags));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_book_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_book_by_id(context::current(), id).await {
            match rpc_result {
                Ok(book) => return Ok(response::okay_with_json(&book)),
                Err(Error::NotFound) => return Ok(response::not_found("BOOK_ID_NOT_FOUND")),
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_author_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_author_by_id(context::current(), id).await {
            match rpc_result {
                Ok(author) => return Ok(response::okay_with_json(&author)),
                Err(Error::NotFound) => return Ok(response::not_found("AUTHOR_ID_NOT_FOUND")),
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_category_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_category_by_id(context::current(), id).await {
            match rpc_result {
                Ok(category) => return Ok(response::okay_with_json(&category)),
                Err(Error::NotFound) => return Ok(response::not_found("CATEGORY_ID_NOT_FOUND")),
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_copy_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_copy_by_id(context::current(), id).await {
            match rpc_result {
                Ok(copy) => return Ok(response::okay_with_json(&copy)),
                Err(Error::NotFound) => return Ok(response::not_found("TAG_ID_NOT_FOUND")),
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_editor_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_editor_by_id(context::current(), id).await {
            match rpc_result {
                Ok(editor) => return Ok(response::okay_with_json(&editor)),
                Err(Error::NotFound) => return Ok(response::not_found("EDITOR_ID_NOT_FOUND")),
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_language_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_language_by_id(context::current(), id).await {
            match rpc_result {
                Ok(language) => return Ok(response::okay_with_json(&language)),
                Err(Error::NotFound) => return Ok(response::not_found("LANGUAGE_ID_NOT_FOUND")),
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_publisher_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_publisher_by_id(context::current(), id).await {
            match rpc_result {
                Ok(publisher) => return Ok(response::okay_with_json(&publisher)),
                Err(Error::NotFound) => return Ok(response::not_found("PUBLISHER_ID_NOT_FOUND")),
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_series_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_series_by_id(context::current(), id).await {
            match rpc_result {
                Ok(series) => return Ok(response::okay_with_json(&series)),
                Err(Error::NotFound) => return Ok(response::not_found("SERIES_ID_NOT_FOUND")),
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_subject_area_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_subject_area_by_id(context::current(), id).await {
            match rpc_result {
                Ok(subject_area) => return Ok(response::okay_with_json(&subject_area)),
                Err(Error::NotFound) => {
                    return Ok(response::not_found("SUBJECT_AREA_ID_NOT_FOUND"))
                }
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_tag_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_tag_by_id(context::current(), id).await {
            match rpc_result {
                Ok(tag) => return Ok(response::okay_with_json(&tag)),
                Err(Error::NotFound) => return Ok(response::not_found("TAG_ID_NOT_FOUND")),
                _ => {}
            }
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_authors_by_book_id(
    id: Uuid,
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(authors) = client
            .get_authors_by_book_id(context::current(), id, page)
            .await
        {
            return Ok(response::okay_with_json(&authors));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_category_by_book_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(category) = client.get_category_by_book_id(context::current(), id).await {
            return Ok(response::okay_with_json(&category));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_copies_by_book_id(
    id: Uuid,
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(copies) = client
            .get_copies_by_book_id(context::current(), id, page)
            .await
        {
            return Ok(response::okay_with_json(&copies));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_editors_by_book_id(
    id: Uuid,
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(editors) = client
            .get_editors_by_book_id(context::current(), id, page)
            .await
        {
            return Ok(response::okay_with_json(&editors));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_language_by_book_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(language) = client.get_language_by_book_id(context::current(), id).await {
            return Ok(response::okay_with_json(&language));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_publisher_by_book_id(
    id: Uuid,
    addr: SocketAddr,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(publisher) = client
            .get_publisher_by_book_id(context::current(), id)
            .await
        {
            return Ok(response::okay_with_json(&publisher));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_series_by_book_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(series) = client.get_series_by_book_id(context::current(), id).await {
            return Ok(response::okay_with_json(&series));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_subject_areas_by_book_id(
    id: Uuid,
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(authors) = client
            .get_subject_areas_by_book_id(context::current(), id, page)
            .await
        {
            return Ok(response::okay_with_json(&authors));
        }
    }
    Ok(response::internal_server_error())
}

pub async fn get_tags_by_book_id(
    id: Uuid,
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Infallible> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(tags) = client
            .get_tags_by_book_id(context::current(), id, page)
            .await
        {
            return Ok(response::okay_with_json(&tags));
        }
    }
    Ok(response::internal_server_error())
}
