use crate::{
    db::{db_connection::Db, queries::book_endpoint::*},
    query_models::{book::*, publisher::*},
};
use std::convert::Infallible;

pub async fn book_list(query: BookQuery, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_books(query, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn create_book(
    query: NewBookQuery,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = add_new_book(query, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

//TODO check`copies` and `borrow` / curently doesn't delete books with copies because of 'ON DELETE RESTRICT' in schema
pub async fn delete_book(book_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    remove_book(book_id, &db_state).unwrap();
    Ok(warp::http::status::StatusCode::OK)
}

pub async fn book_retrieve(book_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_designations(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_designations(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_tags(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_tags(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_publishers(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_publishers(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_authors(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_authors(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_editors(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_editors(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_series(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_series(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_languages(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_languages(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_physical_sizes(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_physical_sizes(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_subject_areas(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_subject_areas(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_copies(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_copies(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_status(
    book_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_book_status(book_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn designations_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_all_designations(&db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn designations_retrieve(
    designation_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_designation_by_id(designation_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn tags_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_all_tags(&db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn tags_retrieve(tag_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_tag_by_id(tag_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn publishers_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_all_publishers(&db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn publishers_retrieve(
    publisher_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_publisher_by_id(publisher_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn create_publisher(
    query: PublisherRequest,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = add_new_publisher(query, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn authors_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_all_authors(&db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn authors_retrieve(
    author_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_author_by_id(author_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn editors_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_all_editors(&db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn editors_retrieve(
    editor_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_editor_by_id(editor_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn series_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_all_series(&db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn series_retrieve(series_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_series_by_id(series_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn languages_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_all_languages(&db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn languages_retrieve(
    language_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_language_by_id(language_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn physical_sizes_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_all_physical_sizes(&db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn physical_sizes_retrieve(
    physical_size_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_physical_size_by_id(physical_size_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn subject_areas_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    let reply = find_all_subject_areas(&db_state).unwrap();
    Ok(warp::reply::json(&reply))
}

pub async fn subject_areas_retrieve(
    subject_area_id: u32,
    db_state: Db,
) -> Result<impl warp::Reply, Infallible> {
    let reply = find_subject_area_by_id(subject_area_id, &db_state).unwrap();
    Ok(warp::reply::json(&reply))
}
