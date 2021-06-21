use std::net::SocketAddr;

use tarpc::context;
use uuid::Uuid;
use warp::{reject::Rejection, Reply};

use book::init_rpc_client;
use helpers::{filters, rpc::Error};

use crate::{
    rejections::{not_found, InternalServerError},
    responses::{json_object_reply, json_vector_reply},
};

pub async fn get_books(
    addr: SocketAddr,
    page: filters::Page,
    book: filters::Book,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(books) = client.get_books(context::current(), page, book).await {
            return Ok(json_vector_reply(&books.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_authors(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(authors) = client.get_authors(context::current(), page).await {
            return Ok(json_vector_reply(&authors.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_categories(
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(categories) = client.get_categories(context::current(), page).await {
            return Ok(json_vector_reply(&categories.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_copies(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(copies) = client.get_copies(context::current(), page).await {
            return Ok(json_vector_reply(&copies.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_editors(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(editors) = client.get_editors(context::current(), page).await {
            return Ok(json_vector_reply(&editors.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_languages(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(languages) = client.get_languages(context::current(), page).await {
            return Ok(json_vector_reply(&languages.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_publishers(
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(publishers) = client.get_publishers(context::current(), page).await {
            return Ok(json_vector_reply(&publishers.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_series(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(series) = client.get_series(context::current(), page).await {
            return Ok(json_vector_reply(&series.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_subject_areas(
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(subject_areas) = client.get_subject_areas(context::current(), page).await {
            return Ok(json_vector_reply(&subject_areas.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_tags(addr: SocketAddr, page: filters::Page) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(tags) = client.get_tags(context::current(), page).await {
            return Ok(json_vector_reply(&tags.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_book_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_book_by_id(context::current(), id).await {
            match rpc_result {
                Ok(book) => return Ok(json_object_reply(&book)),
                Err(Error::NotFound) => return Err(not_found()),
                _ => {}
            }
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_author_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_author_by_id(context::current(), id).await {
            match rpc_result {
                Ok(author) => return Ok(json_object_reply(&author)),
                Err(Error::NotFound) => return Err(not_found()),
                _ => {}
            }
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_category_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_category_by_id(context::current(), id).await {
            match rpc_result {
                Ok(category) => return Ok(json_object_reply(&category)),
                Err(Error::NotFound) => return Err(not_found()),
                _ => {}
            }
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_copy_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_copy_by_id(context::current(), id).await {
            match rpc_result {
                Ok(copy) => return Ok(json_object_reply(&copy)),
                Err(Error::NotFound) => return Err(not_found()),
                _ => {}
            }
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_editor_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_editor_by_id(context::current(), id).await {
            match rpc_result {
                Ok(editor) => return Ok(json_object_reply(&editor)),
                Err(Error::NotFound) => return Err(not_found()),
                _ => {}
            }
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_language_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_language_by_id(context::current(), id).await {
            match rpc_result {
                Ok(language) => return Ok(json_object_reply(&language)),
                Err(Error::NotFound) => return Err(not_found()),
                _ => {}
            }
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_publisher_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_publisher_by_id(context::current(), id).await {
            match rpc_result {
                Ok(publisher) => return Ok(json_object_reply(&publisher)),
                Err(Error::NotFound) => return Err(not_found()),
                _ => {}
            }
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_series_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_series_by_id(context::current(), id).await {
            match rpc_result {
                Ok(series) => return Ok(json_object_reply(&series)),
                Err(Error::NotFound) => return Err(not_found()),
                _ => {}
            }
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_subject_area_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_subject_area_by_id(context::current(), id).await {
            match rpc_result {
                Ok(subject_area) => return Ok(json_object_reply(&subject_area)),
                Err(Error::NotFound) => return Err(not_found()),
                _ => {}
            }
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_tag_by_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(rpc_result) = client.get_tag_by_id(context::current(), id).await {
            match rpc_result {
                Ok(tag) => return Ok(json_object_reply(&tag)),
                Err(Error::NotFound) => return Err(not_found()),
                _ => {}
            }
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_authors_by_book_id(
    id: Uuid,
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(authors) = client
            .get_authors_by_book_id(context::current(), id, page)
            .await
        {
            return Ok(json_object_reply(&authors));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_category_by_book_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(category) = client.get_category_by_book_id(context::current(), id).await {
            return Ok(json_object_reply(&category));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_copies_by_book_id(
    id: Uuid,
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(copies) = client
            .get_copies_by_book_id(context::current(), id, page)
            .await
        {
            return Ok(json_object_reply(&copies));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_editors_by_book_id(
    id: Uuid,
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(editors) = client
            .get_editors_by_book_id(context::current(), id, page)
            .await
        {
            return Ok(json_object_reply(&editors));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_language_by_book_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(language) = client.get_language_by_book_id(context::current(), id).await {
            return Ok(json_object_reply(&language));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_publisher_by_book_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(publisher) = client
            .get_publisher_by_book_id(context::current(), id)
            .await
        {
            return Ok(json_object_reply(&publisher));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_series_by_book_id(id: Uuid, addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(series) = client.get_series_by_book_id(context::current(), id).await {
            return Ok(json_object_reply(&series));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_subject_areas_by_book_id(
    id: Uuid,
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(authors) = client
            .get_subject_areas_by_book_id(context::current(), id, page)
            .await
        {
            return Ok(json_vector_reply(&authors.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_tags_by_book_id(
    id: Uuid,
    addr: SocketAddr,
    page: filters::Page,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = init_rpc_client(&addr).await {
        if let Ok(tags) = client
            .get_tags_by_book_id(context::current(), id, page)
            .await
        {
            return Ok(json_vector_reply(&tags.unwrap()));
        }
    }
    Err(InternalServerError().into())
}
