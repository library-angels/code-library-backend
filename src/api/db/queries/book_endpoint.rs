use crate::db::db_connection::*;
use crate::db::db_model::*;
use crate::query_models::book::*;
use crate::query_models::publisher::*;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::result::Error;

pub fn find_books(query: BookQuery, db_state: &Db)  -> Result<Vec<Book>, Error> {
    use crate::db::schema::books::dsl::*;

    let connection = db_state.get_connection();
    let all_books = {
        let b = books.into_boxed()
            .limit(query.limit.unwrap_or(20))
            .offset(query.offset.unwrap_or(0));
        let b = if let Some(p) = query.publisher_id {
            b.filter(publisher_id.eq(p)) } else { b };
            b.load::<Book>(&connection).expect("can't load the list of books")
        };
    Ok(all_books)
}

pub fn add_new_book(query: NewBookQuery, db_state: &Db) -> Result<Book, Error> {
    use crate::db::schema::books::dsl::*;

    let connection = db_state.get_connection();
    let new_book = NewBook::from(query);
    let new_book: Book =  diesel::insert_into(books)
        .values(new_book)
        .get_result(&connection)
        .expect("can't create book");
    Ok(new_book)
}

pub fn find_book(book_id: u32, db_state: &Db) -> Result<Book, Error> {
    use crate::db::schema::books::dsl::*;

    books.find(book_id as i32)
        .get_result(&db_state.get_connection())

}

pub fn remove_book(book_id: u32, db_state: &Db) -> Result<(), Error> {
    use crate::db::schema::books::dsl::*;
    diesel::delete(books.find(book_id as i32))
        .execute(&db_state.get_connection()).map(|_| ())
}

pub fn find_book_designations(book_id: u32, db_state: &Db) -> Result<Vec<Designation>, Error> {
    use crate::db::schema::books::dsl::{books, id};
    use crate::db::schema::designations::dsl::*;

    let connection = db_state.get_connection();
    let book_designations = books.into_boxed()
        .filter(id.eq(book_id as i32))
        .inner_join(designations)
        .select(designations::all_columns())
        .load::<Designation>(&connection).expect("can't load designations");
    Ok(book_designations)
}

pub fn find_book_tags(b_id: u32, db_state: &Db) -> Result<Vec<BooksTag>, Error> {
    use crate::db::schema::books_tags::dsl::*;
    use crate::db::schema::books::dsl::{id, books};

    let connection = &db_state.get_connection();
    let book_tags = books.filter(id.eq(b_id as i32))
        .inner_join(books_tags)
        .select(books_tags::all_columns())
        .load::<BooksTag>(connection)
        .expect("can't load tags for this book");
    Ok(book_tags)
}

pub fn find_book_publishers(b_id: u32, db_state: &Db) -> Result<Vec<Publisher>, Error> {
    use crate::db::schema::publishers::dsl::*;
    use crate::db::schema::books::dsl::{id, books};

    let connection = &db_state.get_connection();
    let book_publishers = books.filter(id.eq(b_id as i32))
        .inner_join(publishers)
        .select(publishers::all_columns())
        .load::<Publisher>(connection)
        .expect("can't load publishers for this book");
    Ok(book_publishers)
}

pub fn find_book_authors(b_id: u32, db_state: &Db) -> Result<Vec<Person>, Error> {
    use crate::db::schema::books_authors::dsl::*;
    use crate::db::schema::persons::dsl::*;

    let connection = &db_state.get_connection();
    let book_authors = books_authors.filter(book_id.eq(b_id as i32))
        .inner_join(persons)
        .select(persons::all_columns())
        .load::<Person>(connection)
        .expect("can't load authors for this book");
    Ok(book_authors)
}

pub fn find_book_editors(b_id: u32, db_state: &Db) -> Result<Vec<Person>, Error> {
    use crate::db::schema::books_editors::dsl::*;
    use crate::db::schema::persons::dsl::*;

    let connection = &db_state.get_connection();
    let book_editors = books_editors.filter(book_id.eq(b_id as i32))
        .inner_join(persons)
        .select(persons::all_columns())
        .load::<Person>(connection)
        .expect("can't load editors for this book");
    Ok(book_editors)
}

pub fn find_book_series(b_id: u32, db_state: &Db) -> Result<Vec<Series>, Error> {
    use crate::db::schema::series::dsl::*;
    use crate::db::schema::books::dsl::{id, books};

    let connection = &db_state.get_connection();
    let book_series = books.filter(id.eq(b_id as i32))
        .inner_join(series)
        .select(series::all_columns())
        .load::<Series>(connection)
        .expect("can't load series for this book");
    Ok(book_series)
}

pub fn find_book_languages(b_id: u32, db_state: &Db) -> Result<Vec<Language>, Error> {
    use crate::db::schema::languages::dsl::*;
    use crate::db::schema::books::dsl::{id, books};

    let connection = &db_state.get_connection();
    let book_languages = books.filter(id.eq(b_id as i32))
        .inner_join(languages)
        .select(languages::all_columns())
        .load::<Language>(connection)
        .expect("can't load languages for this book");
    Ok(book_languages)
}

pub fn find_book_physical_sizes(b_id: u32, db_state: &Db) -> Result<Vec<PhysicalSize>, Error> {
    use crate::db::schema::physical_sizes::dsl::*;
    use crate::db::schema::books::dsl::{id, books};

    let connection = &db_state.get_connection();
    let book_physical_sizes = books.filter(id.eq(b_id as i32))
        .inner_join(physical_sizes)
        .select(physical_sizes::all_columns())
        .load::<PhysicalSize>(connection)
        .expect("can't load physical size for this book");
    Ok(book_physical_sizes)
}

pub fn find_book_subject_areas(b_id: u32, db_state: &Db) -> Result<Vec<SubjectArea>, Error> {
    use crate::db::schema::books_subject_areas::dsl::*;
    use crate::db::schema::subject_areas::dsl::*;

    let connection = &db_state.get_connection();
    let book_subject_areas = books_subject_areas.filter(book_id.eq(b_id as i32))
        .inner_join(subject_areas)
        .select(subject_areas::all_columns())
        .load::<SubjectArea>(connection)
        .expect("can't load subject areas for this book");
    Ok(book_subject_areas)
}

pub fn find_book_copies(b_id: u32, db_state: &Db) -> Result<Vec<Copy>, Error> {
    use crate::db::schema::copies::dsl::*;

    let connection = &db_state.get_connection();
    let book_copies = copies.filter(book_id.eq(b_id as i32))
        .load::<Copy>(connection)
        .expect("can't load copies for this book");
    Ok(book_copies)
}

pub fn find_book_status(b_id: u32, db_state: &Db) -> Result<Vec<Status>, Error> {
    use crate::db::schema::statuses::dsl::*;
    use crate::db::schema::copies::dsl::*;

    let connection = &db_state.get_connection();
    let book_status = copies.filter(book_id.eq(b_id as i32))
        .inner_join(statuses)
        .select(statuses::all_columns())
        .load::<Status>(connection)
        .expect("can't load status for this book");
    Ok(book_status)
}

pub fn find_all_designations (db_state: &Db) -> Result<Vec<Designation>, Error> {
    use crate::db::schema::designations::dsl::*;

    let connection = &db_state.get_connection();
    let list_of_designations = designations
        .load::<Designation>(connection)
        .expect("can't load list of designations");
    Ok(list_of_designations)
}

pub fn find_designation_by_id(designation_id: u32, db_state: &Db) -> Result<Vec<Designation>, Error> {
    use crate::db::schema::designations::dsl::*;

    let connection = &db_state.get_connection();
    let designations_by_id = designations.filter(id.eq(designation_id as i32))
        .load::<Designation>(connection)
        .expect("can't load this designation");
    Ok(designations_by_id)
}

pub fn find_all_tags(db_state: &Db) -> Result<Vec<Tag>, Error> {
    use crate::db::schema::tags::dsl::*;

    let connection = &db_state.get_connection();
    let all_tags = tags.load::<Tag>(connection).expect("can't load list of tags");
    Ok(all_tags)
}

pub fn find_tag_by_id(tag_id: u32, db_state: &Db) -> Result<Vec<Tag>, Error> {
    use crate::db::schema::tags::dsl::*;

    let connection = &db_state.get_connection();
    let tags_by_id = tags.filter(id.eq(tag_id as i32)) 
        .load::<Tag>(connection)
        .expect("can't load tag");
    Ok(tags_by_id)
}


pub fn find_all_publishers(db_state: &Db) -> Result<Vec<Publisher>, Error> {
    use crate::db::schema::publishers::dsl::*;

    let connection = &db_state.get_connection();
    let list_of_publishers = publishers.load::<Publisher>(connection)
        .expect("can't load list of publishers");
    Ok(list_of_publishers)
}

pub fn find_publisher_by_id(publisher_id: u32, db_state: &Db) -> Result<Vec<Publisher>, Error> {
    use crate::db::schema::publishers::dsl::*;

    let connection = &db_state.get_connection();
    let publisher_by_id = publishers.filter(id.eq(publisher_id as i32))
        .load::<Publisher>(connection)
        .expect("can't load list of publishers");
    Ok(publisher_by_id)
}

pub fn add_new_publisher(query: PublisherRequest, db_state: &Db) -> Result<Publisher, Error> {
    use crate::db::schema::publishers::dsl::*;

    let connection = &db_state.get_connection();
    let new_publisher = NewPublisher::from(query);
    let new_publisher: Publisher = diesel::insert_into(publishers)
        .values(new_publisher)
        .get_result(connection)
        .expect("can't insert new publisher");

    Ok(new_publisher)
}

pub fn find_all_authors(db_state: &Db) -> Result<Vec<Person>, Error> {
    use crate::db::schema::books_authors::dsl::*;
    use crate::db::schema::persons::dsl::*;

    let connection = &db_state.get_connection();
    let list_of_authors = books_authors
        .inner_join(persons)
        .select(persons::all_columns())
        .load::<Person>(connection)
        .expect("can't load authors for this book");
    Ok(list_of_authors)
}

pub fn find_author_by_id(author_id: u32, db_state: &Db) -> Result<Vec<Person>, Error> {
    use crate::db::schema::books_authors::dsl::*;
    use crate::db::schema::persons::dsl::*;

    let connection = &db_state.get_connection();
    let authors_by_id = books_authors.filter(person_id.eq(author_id as i32))
        .inner_join(persons)
        .select(persons::all_columns())
        .load::<Person>(connection)
        .expect("can't load this author");
    Ok(authors_by_id)
}
pub fn find_all_editors(db_state: &Db) -> Result<Vec<BooksEditor>, Error> {
    use crate::db::schema::books_editors::dsl::*;

    let connection = &db_state.get_connection();
    let list_of_editors = books_editors
        .load::<BooksEditor>(connection)
        .expect("can't load list of books editors");
    Ok(list_of_editors)
}

pub fn find_editor_by_id(editor_id: u32, db_state: &Db) -> Result<Vec<BooksEditor>, Error> {
    use crate::db::schema::books_editors::dsl::*;

    let connection = &db_state.get_connection();
    let editors_by_id = books_editors.filter(id.eq(editor_id as i32))
        .load::<BooksEditor>(connection)
        .expect("can't load this editor");
    Ok(editors_by_id)
}

pub fn find_all_series(db_state: &Db) -> Result<Vec<Series>, Error> {
    use crate::db::schema::series::dsl::*;

    let connection = &db_state.get_connection();
    let list_of_series = series
        .load::<Series>(connection)
        .expect("can't load list of series");
    Ok(list_of_series)

}

pub fn find_series_by_id(series_id: u32, db_state: &Db) -> Result<Vec<Series>, Error> {
    use crate::db::schema::series::dsl::*;

    let connection = &db_state.get_connection();
    let series_by_id = series.filter(id.eq(series_id as i32))
        .load::<Series>(connection)
        .expect("can't load this series");
    Ok(series_by_id)
}

pub fn find_all_languages(db_state: &Db) -> Result<Vec<Language>, Error> {
    use crate::db::schema::languages::dsl::*;

    let connection = &db_state.get_connection();
    let list_of_languages = languages
        .load::<Language>(connection)
        .expect("can't load list of languages");
    Ok(list_of_languages)

}

pub fn find_language_by_id(language_id: u32, db_state: &Db) -> Result<Vec<Language>, Error> {
    use crate::db::schema::languages::dsl::*;

    let connection = &db_state.get_connection();
    let languages_by_id = languages.filter(id.eq(language_id as i32))
        .load::<Language>(connection)
        .expect("can't load this language");
    Ok(languages_by_id)

}

pub fn find_all_physical_sizes(db_state: &Db) -> Result<Vec<PhysicalSize>, Error> {
    use crate::db::schema::physical_sizes::dsl::*;

    let connection = &db_state.get_connection();
    let list_of_physical_sizes = physical_sizes
        .load::<PhysicalSize>(connection)
        .expect("can't load list of physical sizes");
    Ok(list_of_physical_sizes)
}

pub fn find_physical_size_by_id(physical_size_id: u32, db_state: &Db) -> Result<Vec<PhysicalSize>, Error> {
    use crate::db::schema::physical_sizes::dsl::*;

    let connection = &db_state.get_connection();
    let physical_size_by_id = physical_sizes.filter(id.eq(physical_size_id as i32))
        .load::<PhysicalSize>(connection)
        .expect("can't load this physical size");
    Ok(physical_size_by_id)
}

pub fn find_all_subject_areas(db_state: &Db) -> Result<Vec<SubjectArea>, Error> {
    use crate::db::schema::subject_areas::dsl::*;

    let connection = &db_state.get_connection();
    let list_of_subject_areas = subject_areas
        .load::<SubjectArea>(connection)
        .expect("can't load list of subject areas");
    Ok(list_of_subject_areas)
}

pub fn find_subject_area_by_id(subject_area_id: u32, db_state: &Db) -> Result<Vec<SubjectArea>, Error> {
    use crate::db::schema::subject_areas::dsl::*;

    let connection = &db_state.get_connection();
    let subject_area_by_id = subject_areas.filter(id.eq(subject_area_id as i32))
        .load::<SubjectArea>(connection)
        .expect("can't load this subject area");
    Ok(subject_area_by_id)
}
