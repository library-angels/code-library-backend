use super::service::*;
use std::net::SocketAddr;
use tarpc::context;

#[derive(Clone)]
pub struct BookServer(pub SocketAddr);

#[tarpc::server]
impl BookService for BookServer {
    async fn create_book(self, _: context::Context, _book: Book) -> Result<Book, Error> {
        unimplemented!();
    }

    async fn get_book(self, _: context::Context, _book_id: u32) -> Result<Book, Error> {
        unimplemented!();
    }

    async fn list_books(
        self,
        _: context::Context,
        _book_filter: BookFilter,
        _sort: Option<SortField>,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Book>, Error> {
        unimplemented!();
    }

    async fn update_book(self, _: context::Context, _book: Book) -> Result<Book, Error> {
        unimplemented!();
    }

    async fn get_book_category(
        self,
        _: context::Context,
        _book_id: u32,
        _offset: u32,
        _limit: u32,
    ) -> Result<Category, Error> {
        unimplemented!();
    }

    async fn get_book_publisher(
        self,
        _: context::Context,
        _book_id: u32,
        _offset: u32,
        _limit: u32,
    ) -> Result<Publisher, Error> {
        unimplemented!();
    }

    async fn list_book_tags(
        self,
        _: context::Context,
        _book_id: u32,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Tag>, Error> {
        unimplemented!();
    }

    async fn list_book_authors(
        self,
        _: context::Context,
        _book_id: u32,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Author>, Error> {
        unimplemented!();
    }

    async fn list_book_editors(
        self,
        _: context::Context,
        _book_id: u32,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Editor>, Error> {
        unimplemented!();
    }

    async fn get_book_series(
        self,
        _: context::Context,
        _book_id: u32,
        _offset: u32,
        _limit: u32,
    ) -> Result<Series, Error> {
        unimplemented!();
    }

    async fn get_book_language(
        self,
        _: context::Context,
        _book_id: u32,
        _offset: u32,
        _limit: u32,
    ) -> Result<Language, Error> {
        unimplemented!();
    }

    async fn get_book_physical_size(
        self,
        _: context::Context,
        _book_id: u32,
        _offset: u32,
        _limit: u32,
    ) -> Result<PhysicalSize, Error> {
        unimplemented!();
    }

    async fn get_book_subject_area(
        self,
        _: context::Context,
        _book_id: u32,
        _offset: u32,
        _limit: u32,
    ) -> Result<SubjectArea, Error> {
        unimplemented!();
    }

    async fn create_book_instance(
        self,
        _: context::Context,
        _book_instance: Instance,
    ) -> Result<Instance, Error> {
        unimplemented!();
    }

    async fn list_book_instances(
        self,
        _: context::Context,
        _book_id: u32,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Instance>, Error> {
        unimplemented!();
    }

    async fn get_category(self, _: context::Context, _category_id: u32) -> Result<Category, Error> {
        unimplemented!();
    }

    async fn list_categories(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Category>, Error> {
        unimplemented!();
    }

    async fn get_tag(self, _: context::Context, _tag_id: u32) -> Result<Tag, Error> {
        unimplemented!();
    }

    async fn list_tags(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Tag>, Error> {
        unimplemented!();
    }

    async fn get_publisher(
        self,
        _: context::Context,
        _publisher_id: u32,
    ) -> Result<Publisher, Error> {
        unimplemented!();
    }

    async fn list_publishers(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Publisher>, Error> {
        unimplemented!();
    }

    async fn get_author(self, _: context::Context, _author_id: u32) -> Result<Author, Error> {
        unimplemented!();
    }

    async fn list_authors(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Author>, Error> {
        unimplemented!();
    }

    async fn get_editor(self, _: context::Context, _editor_id: u32) -> Result<Editor, Error> {
        unimplemented!();
    }

    async fn list_editors(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Editor>, Error> {
        unimplemented!();
    }

    async fn get_series(self, _: context::Context, _series_id: u32) -> Result<Series, Error> {
        unimplemented!();
    }

    async fn list_series(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Series>, Error> {
        unimplemented!();
    }

    async fn get_language(self, _: context::Context, _language_id: u32) -> Result<Language, Error> {
        unimplemented!();
    }

    async fn list_languages(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<Language>, Error> {
        unimplemented!();
    }

    async fn get_physical_size(
        self,
        _: context::Context,
        _physical_size_id: u32,
    ) -> Result<PhysicalSize, Error> {
        unimplemented!();
    }

    async fn list_physical_sizes(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<PhysicalSize>, Error> {
        unimplemented!();
    }

    async fn get_subject_area(
        self,
        _: context::Context,
        _subject_area_id: u32,
    ) -> Result<SubjectArea, Error> {
        unimplemented!();
    }

    async fn list_subject_areas(
        self,
        _: context::Context,
        _offset: u32,
        _limit: u32,
    ) -> Result<Vec<SubjectArea>, Error> {
        unimplemented!();
    }
}
