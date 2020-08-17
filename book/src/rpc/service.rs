use serde::{Deserialize, Serialize};

#[tarpc::service]
pub trait BookService {
    async fn create_book(book: Book) -> Result<Book, Error>;
    async fn get_book(book_id: u32) -> Result<Book, Error>;
    async fn list_books(
        categories: Option<Vec<Category>>,
        tags: Option<Vec<Tag>>,
        series: Option<Vec<Series>>,
        publishers: Option<Vec<Publisher>>,
        search_field: Option<Vec<SearchField>>,
        search_keywords: Option<Vec<String>>,
        sort: Option<SortField>,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<Book>, Error>;
    async fn update_book(book: Book) -> Result<Book, Error>;
    async fn get_book_category(book_id: u32, offset: u32, limit: u32) -> Result<Category, Error>;
    async fn get_book_publisher(book_id: u32, offset: u32, limit: u32) -> Result<Publisher, Error>;
    async fn list_book_tags(book_id: u32, offset: u32, limit: u32) -> Result<Vec<Tag>, Error>;
    async fn list_book_authors(book_id: u32, offset: u32, limit: u32)
        -> Result<Vec<Author>, Error>;
    async fn list_book_editors(book_id: u32, offset: u32, limit: u32)
        -> Result<Vec<Editor>, Error>;
    async fn get_book_series(book_id: u32, offset: u32, limit: u32) -> Result<Series, Error>;
    async fn get_book_language(book_id: u32, offset: u32, limit: u32) -> Result<Language, Error>;
    async fn get_book_physical_size(
        book_id: u32,
        offset: u32,
        limit: u32,
    ) -> Result<PhysicalSize, Error>;
    async fn get_book_subject_area(
        book_id: u32,
        offset: u32,
        limit: u32,
    ) -> Result<SubjectArea, Error>;
    async fn create_book_instance(book_instance: Instance) -> Result<Instance, Error>;
    async fn list_book_instances(
        book_id: u32,
        offset: u32,
        limit: u32,
    ) -> Result<Vec<Instance>, Error>;
    async fn get_category(category_id: u32) -> Result<Category, Error>;
    async fn list_categories(offset: u32, limit: u32) -> Result<Vec<Category>, Error>;
    async fn get_tag(tag_id: u32) -> Result<Tag, Error>;
    async fn list_tags(offset: u32, limit: u32) -> Result<Vec<Tag>, Error>;
    async fn get_publisher(publisher_id: u32) -> Result<Publisher, Error>;
    async fn list_publishers(offset: u32, limit: u32) -> Result<Vec<Publisher>, Error>;
    async fn get_author(author_id: u32) -> Result<Author, Error>;
    async fn list_authors(offset: u32, limit: u32) -> Result<Vec<Author>, Error>;
    async fn get_editor(editor_id: u32) -> Result<Editor, Error>;
    async fn list_editors(offset: u32, limit: u32) -> Result<Vec<Editor>, Error>;
    async fn get_series(series_id: u32) -> Result<Series, Error>;
    async fn list_series(offset: u32, limit: u32) -> Result<Vec<Series>, Error>;
    async fn get_language(language_id: u32) -> Result<Language, Error>;
    async fn list_languages(offset: u32, limit: u32) -> Result<Vec<Language>, Error>;
    async fn get_physical_size(physical_size_id: u32) -> Result<PhysicalSize, Error>;
    async fn list_physical_sizes(offset: u32, limit: u32) -> Result<Vec<PhysicalSize>, Error>;
    async fn get_subject_area(subject_area_id: u32) -> Result<SubjectArea, Error>;
    async fn list_subject_areas(offset: u32, limit: u32) -> Result<Vec<SubjectArea>, Error>;
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    NotFound,
    AlreadyExists,
    InvalidInput,
    InvalidData,
    InternalError,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SearchField {
    All,
    Title,
    Author,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SortField {
    AlphaAsc,
    AlphaDesc,
    ReleaseDateAsc,
    ReleaseDateDesc,
    CodeIdentifierAsc,
    CodeIdentifierDesc,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: u32,
    pub isbn_13: String,
    pub issn: String,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub cover: u32,
    pub edition: u32,
    pub release_year: u32,
    pub pages: u32,
    pub code_identifier: String,
    pub publisher_id: u32,
    pub category_id: u32,
    pub series_id: u32,
    pub language_id: u32,
    pub physical_size_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Publisher {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub id: u32,
    pub given_name: String,
    pub family_name: String,
    pub date_of_birth: u64,
    pub isni: String,
    pub orcid: String,
    pub oclc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Editor {
    pub id: u32,
    pub given_name: String,
    pub family_name: String,
    pub date_of_birth: u64,
    pub isni: String,
    pub orcid: String,
    pub oclc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Series {
    pub id: u32,
    pub name: String,
    pub publisher_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Language {
    pub id: u32,
    pub name: String,
    pub iso_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalSize {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectArea {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Instance {
    pub id: u32,
    pub book_id: u32,
    pub code_identifier_copy_id: u32,
    pub date_added: u64,
}
