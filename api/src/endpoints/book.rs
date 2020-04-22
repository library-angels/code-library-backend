use crate::query_models::publisher::*;
use crate::query_models::book::*;
use crate::db::db_connection::{Db};
use std::convert::Infallible;
use diesel::query_dsl::QueryDsl;
use diesel::prelude::*;

pub async fn book_list(query: BookQuery, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Book;
    use crate::db::schema::books::dsl::*;

    let connection = &db_state.get_connection();
    let reply = { 
        let b = books.into_boxed()
            .limit(query.limit.unwrap_or(20))
            .offset(query.offset.unwrap_or(0));
        let b = if let Some(p) = query.publisher_id {
            b.filter(publisher_id.eq(p)) } else { b };
            b.load::<Book>(connection).expect("can't load the list of books")
    };
    Ok(warp::reply::json(&reply))
}

pub async fn create_book(query: NewBookQuery, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Book;
    use crate::db::schema::books::dsl::*;

    let connection = &db_state.get_connection();
    let new_book = NewBook::from(query);
    let new_book: Book =  diesel::insert_into(books)
        .values(new_book)
        .get_result(connection)
        .expect("can't create book");
    Ok(warp::reply::json(&new_book))
}

pub async fn delete_book(book_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::schema::books::dsl::*;

    let connection = &db_state.get_connection();
    let book = books.filter(id.eq(book_id as i32));
    diesel::delete(book).execute(connection).expect("can't delete book");
    Ok(warp::http::status::StatusCode::OK)
}

pub async fn book_retrieve(book_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Book;
    use crate::db::schema::books::dsl::*;

    let connection = &db_state.get_connection();
    let reply = books.filter(id.eq(book_id as i32))
        .load::<Book>(connection)
        .expect("can't load this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_designations(book_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Designation;
    use crate::db::schema::books::dsl::{books, id};
    use crate::db::schema::designations::dsl::*;

    let connection = &db_state.get_connection();
    let reply = books.filter(id.eq(book_id as i32))
        .inner_join(designations)
        .select(designations::all_columns())
        .load::<Designation>(connection)
        .expect("can't load designations for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_tags(b_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::BooksTag;
    use crate::db::schema::books_tags::dsl::*;
    use crate::db::schema::books::dsl::{id, books};

    let connection = &db_state.get_connection();
    let reply = books.filter(id.eq(b_id as i32))
        .inner_join(books_tags)
        .select(books_tags::all_columns())
        .load::<BooksTag>(connection)
        .expect("can't load tags for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_publishers(b_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Publisher;
    use crate::db::schema::publishers::dsl::*;
    use crate::db::schema::books::dsl::{id, books};

    let connection = &db_state.get_connection();
    let reply = books.filter(id.eq(b_id as i32))
        .inner_join(publishers)
        .select(publishers::all_columns())
        .load::<Publisher>(connection)
        .expect("can't load publishers for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_authors(b_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Person;
    use crate::db::schema::books_authors::dsl::*;
    use crate::db::schema::persons::dsl::*;

    let connection = &db_state.get_connection();
    let reply = books_authors.filter(book_id.eq(b_id as i32))
        .inner_join(persons)
        .select(persons::all_columns())
        .load::<Person>(connection)
        .expect("can't load authors for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_editors(b_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Person;
    use crate::db::schema::books_editors::dsl::*;
    use crate::db::schema::persons::dsl::*;

    let connection = &db_state.get_connection();
    let reply = books_editors.filter(book_id.eq(b_id as i32))
        .inner_join(persons)
        .select(persons::all_columns())
        .load::<Person>(connection)
        .expect("can't load editors for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_series(b_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Series;
    use crate::db::schema::series::dsl::*;
    use crate::db::schema::books::dsl::{id, books};

    let connection = &db_state.get_connection();
    let reply = books.filter(id.eq(b_id as i32))
        .inner_join(series)
        .select(series::all_columns())
        .load::<Series>(connection)
        .expect("can't load series for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_languages(b_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Language;
    use crate::db::schema::languages::dsl::*;
    use crate::db::schema::books::dsl::{id, books};

    let connection = &db_state.get_connection();
    let reply = books.filter(id.eq(b_id as i32))
        .inner_join(languages)
        .select(languages::all_columns())
        .load::<Language>(connection)
        .expect("can't load languages for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_physical_sizes(b_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::PhysicalSize;
    use crate::db::schema::physical_sizes::dsl::*;
    use crate::db::schema::books::dsl::{id, books};

    let connection = &db_state.get_connection();
    let reply = books.filter(id.eq(b_id as i32))
        .inner_join(physical_sizes)
        .select(physical_sizes::all_columns())
        .load::<PhysicalSize>(connection)
        .expect("can't load physical size for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_subject_areas(b_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::SubjectArea;
    use crate::db::schema::books_subject_areas::dsl::*;
    use crate::db::schema::subject_areas::dsl::*;

    let connection = &db_state.get_connection();
    let reply = books_subject_areas.filter(book_id.eq(b_id as i32))
        .inner_join(subject_areas)
        .select(subject_areas::all_columns())
        .load::<SubjectArea>(connection)
        .expect("can't load subject areas for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_copies(b_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Copy;
    use crate::db::schema::copies::dsl::*;

    let connection = &db_state.get_connection();
    let reply = copies.filter(book_id.eq(b_id as i32))
        .load::<Copy>(connection)
        .expect("can't load copies for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn book_retrieve_status(b_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Status;
    use crate::db::schema::statuses::dsl::*;
    use crate::db::schema::copies::dsl::*;

    let connection = &db_state.get_connection();
    let reply = copies.filter(book_id.eq(b_id as i32))
        .inner_join(statuses)
        .select(statuses::all_columns())
        .load::<Status>(connection)
        .expect("can't load status for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn designations_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Designation;
    use crate::db::schema::designations::dsl::*;

    let connection = &db_state.get_connection();
    let reply = designations
        .load::<Designation>(connection)
        .expect("can't load list of designations");
    Ok(warp::reply::json(&reply))
}

pub async fn designations_retrieve(designation_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Designation;
    use crate::db::schema::designations::dsl::*;

    let connection = &db_state.get_connection();
    let reply = designations.filter(id.eq(designation_id as i32))
        .load::<Designation>(connection)
        .expect("can't load this designation");
    Ok(warp::reply::json(&reply))
}

pub async fn tags_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Tag;
    use crate::db::schema::tags::dsl::*;

    let connection = &db_state.get_connection();
    let reply = tags.load::<Tag>(connection).expect("can't load list of tags");
    Ok(warp::reply::json(&reply))
    
}

pub async fn tags_retrieve(tag_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Tag;
    use crate::db::schema::tags::dsl::*;

    let connection = &db_state.get_connection();
    let reply = tags.filter(id.eq(tag_id as i32)) 
        .load::<Tag>(connection)
        .expect("can't load tag");
    Ok(warp::reply::json(&reply))
}

pub async fn publishers_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Publisher;
    use crate::db::schema::publishers::dsl::*;

    let connection = &db_state.get_connection();
    let reply = publishers 
        .load::<Publisher>(connection).expect("can't load list of publishers");
    Ok(warp::reply::json(&reply))
}

pub async fn publishers_retrieve(publisher_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Publisher;
    use crate::db::schema::publishers::dsl::*;

    let connection = &db_state.get_connection();
    let reply = publishers.filter(id.eq(publisher_id as i32))
        .load::<Publisher>(connection)
        .expect("can't load this publisher");
    Ok(warp::reply::json(&reply))
}

pub async fn create_publisher (query: PublisherRequest, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Publisher;
    use crate::db::schema::publishers::dsl::*;

    let connection = &db_state.get_connection();
    let new_publisher = NewPublisher::from(query);
    let new_publisher: Publisher = diesel::insert_into(publishers)
        .values(new_publisher)
        .get_result(connection)
        .expect("can't insert new publisher");

    Ok(warp::reply::json(&new_publisher))
}

pub async fn authors_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Person;
    use crate::db::schema::books_authors::dsl::*;
    use crate::db::schema::persons::dsl::*;

    let connection = &db_state.get_connection();
    let reply = books_authors
        .inner_join(persons)
        .select(persons::all_columns())
        .load::<Person>(connection)
        .expect("can't load authors for this book");
    Ok(warp::reply::json(&reply))
}

pub async fn authors_retrieve(author_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Person;
    use crate::db::schema::books_authors::dsl::*;
    use crate::db::schema::persons::dsl::*;

    let connection = &db_state.get_connection();
    let reply = books_authors.filter(person_id.eq(author_id as i32))
        .inner_join(persons)
        .select(persons::all_columns())
        .load::<Person>(connection)
        .expect("can't load this author");
    Ok(warp::reply::json(&reply))
}

pub async fn editors_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::BooksEditor;
    use crate::db::schema::books_editors::dsl::*;

    let connection = &db_state.get_connection();
    let reply = books_editors
        .load::<BooksEditor>(connection)
        .expect("can't load list of books editors");
    Ok(warp::reply::json(&reply))
}

pub async fn editors_retrieve(editor_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::BooksEditor;
    use crate::db::schema::books_editors::dsl::*;

    let connection = &db_state.get_connection();
    let reply = books_editors.filter(id.eq(editor_id as i32))
        .load::<BooksEditor>(connection)
        .expect("can't load this editor");
    Ok(warp::reply::json(&reply))
}

pub async fn series_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Series;
    use crate::db::schema::series::dsl::*;

    let connection = &db_state.get_connection();
    let reply = series
        .load::<Series>(connection)
        .expect("can't load list of series");
    Ok(warp::reply::json(&reply))

}

pub async fn series_retrieve(series_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Series;
    use crate::db::schema::series::dsl::*;

    let connection = &db_state.get_connection();
    let reply = series.filter(id.eq(series_id as i32))
        .load::<Series>(connection)
        .expect("can't load this series");
    Ok(warp::reply::json(&reply))
}

pub async fn languages_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Language;
    use crate::db::schema::languages::dsl::*;

    let connection = &db_state.get_connection();
    let reply = languages
        .load::<Language>(connection)
        .expect("can't load list of languages");
    Ok(warp::reply::json(&reply))

}

pub async fn languages_retrieve(language_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::Language;
    use crate::db::schema::languages::dsl::*;

    let connection = &db_state.get_connection();
    let reply = languages.filter(id.eq(language_id as i32))
        .load::<Language>(connection)
        .expect("can't load this language");
    Ok(warp::reply::json(&reply))

}

pub async fn physical_sizes_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::PhysicalSize;
    use crate::db::schema::physical_sizes::dsl::*;

    let connection = &db_state.get_connection();
    let reply = physical_sizes
        .load::<PhysicalSize>(connection)
        .expect("can't load list of physical sizes");
    Ok(warp::reply::json(&reply))
}

pub async fn physical_sizes_retrieve(physical_size_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::PhysicalSize;
    use crate::db::schema::physical_sizes::dsl::*;

    let connection = &db_state.get_connection();
    let reply = physical_sizes.filter(id.eq(physical_size_id as i32))
        .load::<PhysicalSize>(connection)
        .expect("can't load this physical size");
    Ok(warp::reply::json(&reply))
}

pub async fn subject_areas_list(db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::SubjectArea;
    use crate::db::schema::subject_areas::dsl::*;

    let connection = &db_state.get_connection();
    let reply = subject_areas
        .load::<SubjectArea>(connection)
        .expect("can't load list of subject areas");
    Ok(warp::reply::json(&reply))
}

pub async fn subject_areas_retrieve(subject_area_id: u32, db_state: Db) -> Result<impl warp::Reply, Infallible> {
    use crate::db::db_model::SubjectArea;
    use crate::db::schema::subject_areas::dsl::*;

    let connection = &db_state.get_connection();
    let reply = subject_areas.filter(id.eq(subject_area_id as i32))
        .load::<SubjectArea>(connection)
        .expect("can't load this subject area");
    Ok(warp::reply::json(&reply))
}
