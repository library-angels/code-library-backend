use std::convert::Infallible;
use std::collections::HashMap;


pub async fn book_list(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book index"))
}

pub async fn book_retrieve(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {}", book_id))
}

pub async fn book_retrieve_designations(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - designations index", book_id))
}

pub async fn book_retrieve_designations_retrieve(book_id: u32, designation_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - designation id: {}", book_id, designation_id))
}

pub async fn book_retrieve_tags(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - tag index", book_id))
}

pub async fn book_retrieve_tags_retrieve(book_id: u32, tag_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - tag id: {}", book_id, tag_id))
}

pub async fn book_retrieve_publishers(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - publisher index", book_id))
}

pub async fn book_retrieve_publishers_retrieve(book_id: u32, publisher_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - publisher id: {}", book_id, publisher_id))
}

pub async fn book_retrieve_authors(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - author index", book_id))
}

pub async fn book_retrieve_authors_retrieve(book_id: u32, author_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - author id: {}", book_id, author_id))
}

pub async fn book_retrieve_editors(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - editor index", book_id))
}

pub async fn book_retrieve_editors_retrieve(book_id: u32, editor_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - editor id: {}", book_id, editor_id))
}

pub async fn book_retrieve_series(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - series index", book_id))
}

pub async fn book_retrieve_series_retrieve(book_id: u32, series_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - series id: {}", book_id, series_id))
}

pub async fn book_retrieve_languages(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - language index", book_id))
}

pub async fn book_retrieve_languages_retrieve(book_id: u32, language_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - language id: {}", book_id, language_id))
}

pub async fn book_retrieve_physical_sizes(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - physical sizes index", book_id))
}

pub async fn book_retrieve_physical_sizes_retrieve(book_id: u32, physical_size_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - physical sizes id: {}", book_id, physical_size_id))
}

pub async fn book_retrieve_subject_areas(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - subject area index", book_id))
}

pub async fn book_retrieve_subject_areas_retrieve(book_id: u32, subject_area_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - subject area id: {}", book_id, subject_area_id))
}

pub async fn book_retrieve_copies(book_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - copy index", book_id))
}

pub async fn book_retrieve_copies_retrieve(book_id: u32, copy_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("book id: {} - copy id: {}", book_id, copy_id))
}

pub async fn designations_list(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("designations index"))
}

pub async fn designations_retrieve(designation_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("designations id: {}", designation_id))
}

pub async fn tags_list(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("tags index"))
}

pub async fn tags_retrieve(tag_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("tags id: {}", tag_id))
}

pub async fn publishers_list(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("publishers index"))
}

pub async fn publishers_retrieve(publisher_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("publishers id: {}", publisher_id))
}

pub async fn authors_list(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("authors index"))
}

pub async fn authors_retrieve(author_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("authors id: {}", author_id))
}

pub async fn editors_list(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("editors index"))
}

pub async fn editors_retrieve(editor_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("editors id: {}", editor_id))
}

pub async fn series_list(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("series index"))
}

pub async fn series_retrieve(series_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("series id: {}", series_id))
}

pub async fn languages_list(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("languages index"))
}

pub async fn languages_retrieve(language_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("languages id: {}", language_id))
}

pub async fn physical_sizes_list(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("physical sizes index"))
}

pub async fn physical_sizes_retrieve(physical_size_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("physical sizes id: {}", physical_size_id))
}

pub async fn subject_areas_list(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("subject areas index"))
}

pub async fn subject_areas_retrieve(subject_area_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("subject areas id: {}", subject_area_id))
}
