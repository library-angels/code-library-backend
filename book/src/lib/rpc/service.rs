use uuid::Uuid;

use super::models::{
    Author, Book, Category, Copy, Editor, Language, Publisher, Series, SubjectArea, Tag,
};
use helpers::{rpc::RpcResult, types::PageFilter};

#[tarpc::service]
pub trait BookService {
    async fn get_language_by_id(id: Uuid) -> RpcResult<Language>;
    async fn get_language_by_book_id(id: Uuid) -> RpcResult<Language>;
    async fn get_languages(page_filter: PageFilter) -> RpcResult<Vec<Language>>;
    async fn get_category_by_id(id: Uuid) -> RpcResult<Category>;
    async fn get_category_by_book_id(id: Uuid) -> RpcResult<Category>;
    async fn get_categories(page_filter: PageFilter) -> RpcResult<Vec<Category>>;
    async fn get_publisher_by_id(id: Uuid) -> RpcResult<Publisher>;
    async fn get_publisher_by_book_id(id: Uuid) -> RpcResult<Publisher>;
    async fn get_publishers(page_filter: PageFilter) -> RpcResult<Vec<Publisher>>;
    async fn get_series_by_id(id: Uuid) -> RpcResult<Series>;
    async fn get_series_by_book_id(id: Uuid) -> RpcResult<Series>;
    async fn get_series(page_filter: PageFilter) -> RpcResult<Vec<Series>>;
    async fn get_subject_area_by_id(id: Uuid) -> RpcResult<SubjectArea>;
    async fn get_subject_areas(page_filter: PageFilter) -> RpcResult<Vec<SubjectArea>>;
    async fn get_subject_areas_by_book_id(
        id: Uuid,
        page_filter: PageFilter,
    ) -> RpcResult<Vec<SubjectArea>>;
    async fn get_tag_by_id(id: Uuid) -> RpcResult<Tag>;
    async fn get_tags(page_filter: PageFilter) -> RpcResult<Vec<Tag>>;
    async fn get_tags_by_book_id(id: Uuid, page_filter: PageFilter) -> RpcResult<Vec<Tag>>;
    async fn get_author_by_id(id: Uuid) -> RpcResult<Author>;
    async fn get_authors(page_filter: PageFilter) -> RpcResult<Vec<Author>>;
    async fn get_authors_by_book_id(id: Uuid, page_filter: PageFilter) -> RpcResult<Vec<Author>>;
    async fn get_editor_by_id(id: Uuid) -> RpcResult<Editor>;
    async fn get_editors(page_filter: PageFilter) -> RpcResult<Vec<Editor>>;
    async fn get_editors_by_book_id(id: Uuid, page_filter: PageFilter) -> RpcResult<Vec<Editor>>;
    async fn get_copy_by_id(id: Uuid) -> RpcResult<Copy>;
    async fn get_copies(page_filter: PageFilter) -> RpcResult<Vec<Copy>>;
    async fn get_copies_by_book_id(id: Uuid, page_filter: PageFilter) -> RpcResult<Vec<Copy>>;
    async fn get_book_by_id(id: Uuid) -> RpcResult<Book>;
    async fn get_books(page_filter: PageFilter) -> RpcResult<Vec<Book>>;
}
