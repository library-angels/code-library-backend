use sea_query::PostgresQueryBuilder;
use sqlx::query_as;
use tarpc::context::Context;
use uuid::Uuid;

use helpers::{filters, rpc::RpcResult};

use super::models::{
    Author, Book, Category, Copy, Editor, Language, Publisher, Series, SubjectArea, Tag,
};
use super::service::BookService;
use crate::db::{models as db_models, queries, DbConnection, DbPool};

sea_query::sea_query_driver_postgres!();
use sea_query_driver_postgres::bind_query_as;

#[derive(Clone)]
pub struct BookServer {
    db_pool: crate::db::DbPool,
}

impl BookServer {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub async fn get_db_connection(&self) -> DbConnection {
        self.db_pool
            .acquire()
            .await
            .expect("Failed to retrieve connection from the pool")
    }
}

#[tarpc::server]
impl BookService for BookServer {
    async fn get_language_by_id(self, _: Context, id: Uuid) -> RpcResult<Language> {
        let (query, values) = queries::get_language_by_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Language>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_language_by_book_id(self, _: Context, id: Uuid) -> RpcResult<Language> {
        let (query, values) = queries::get_language_by_book_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Language>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_languages(self, _: Context, page: filters::Page) -> RpcResult<Vec<Language>> {
        let (query, values) = queries::get_languages(page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Language>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|language| language.into())
                .collect(),
        )
    }

    async fn get_category_by_id(self, _: Context, id: Uuid) -> RpcResult<Category> {
        let (query, values) = queries::get_category_by_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Category>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_category_by_book_id(self, _: Context, id: Uuid) -> RpcResult<Category> {
        let (query, values) = queries::get_category_by_book_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Category>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_categories(self, _: Context, page: filters::Page) -> RpcResult<Vec<Category>> {
        let (query, values) = queries::get_categories(page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Category>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|category| category.into())
                .collect(),
        )
    }

    async fn get_publisher_by_id(self, _: Context, id: Uuid) -> RpcResult<Publisher> {
        let (query, values) = queries::get_publisher_by_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Publisher>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_publisher_by_book_id(self, _: Context, id: Uuid) -> RpcResult<Publisher> {
        let (query, values) = queries::get_publisher_by_book_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Publisher>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_publishers(self, _: Context, page: filters::Page) -> RpcResult<Vec<Publisher>> {
        let (query, values) = queries::get_publishers(page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Publisher>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|publisher| publisher.into())
                .collect(),
        )
    }

    async fn get_series_by_id(self, _: Context, id: Uuid) -> RpcResult<Series> {
        let (query, values) = queries::get_series_by_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Series>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_series_by_book_id(self, _: Context, id: Uuid) -> RpcResult<Series> {
        let (query, values) = queries::get_series_by_book_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Series>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_series(self, _: Context, page: filters::Page) -> RpcResult<Vec<Series>> {
        let (query, values) = queries::get_series(page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Series>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|series| series.into())
                .collect(),
        )
    }

    async fn get_subject_area_by_id(self, _: Context, id: Uuid) -> RpcResult<SubjectArea> {
        let (query, values) = queries::get_subject_area_by_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::SubjectArea>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_subject_areas(
        self,
        _: Context,
        page: filters::Page,
    ) -> RpcResult<Vec<SubjectArea>> {
        let (query, values) = queries::get_subject_areas(page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::SubjectArea>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|subject_area| subject_area.into())
                .collect(),
        )
    }

    async fn get_subject_areas_by_book_id(
        self,
        _: Context,
        id: Uuid,
        page: filters::Page,
    ) -> RpcResult<Vec<SubjectArea>> {
        let (query, values) =
            queries::get_subject_areas_by_book_id(id, page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::SubjectArea>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|subject_area| subject_area.into())
                .collect(),
        )
    }

    async fn get_tag_by_id(self, _: Context, id: Uuid) -> RpcResult<Tag> {
        let (query, values) = queries::get_tag_by_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Tag>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_tags(self, _: Context, page: filters::Page) -> RpcResult<Vec<Tag>> {
        let (query, values) = queries::get_tags(page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Tag>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|tag| tag.into())
                .collect(),
        )
    }

    async fn get_tags_by_book_id(
        self,
        _: Context,
        id: Uuid,
        page: filters::Page,
    ) -> RpcResult<Vec<Tag>> {
        let (query, values) = queries::get_tags_by_book_id(id, page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Tag>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|tag| tag.into())
                .collect(),
        )
    }

    async fn get_author_by_id(self, _: Context, id: Uuid) -> RpcResult<Author> {
        let (query, values) = queries::get_author_by_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Author>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_authors(self, _: Context, page: filters::Page) -> RpcResult<Vec<Author>> {
        let (query, values) = queries::get_authors(page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Author>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|author| author.into())
                .collect(),
        )
    }

    async fn get_authors_by_book_id(
        self,
        _: Context,
        id: Uuid,
        page: filters::Page,
    ) -> RpcResult<Vec<Author>> {
        let (query, values) = queries::get_authors_by_book_id(id, page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Author>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|author| author.into())
                .collect(),
        )
    }

    async fn get_editor_by_id(self, _: Context, id: Uuid) -> RpcResult<Editor> {
        let (query, values) = queries::get_editor_by_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Editor>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_editors(self, _: Context, page: filters::Page) -> RpcResult<Vec<Editor>> {
        let (query, values) = queries::get_editors(page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Editor>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|editor| editor.into())
                .collect(),
        )
    }

    async fn get_editors_by_book_id(
        self,
        _: Context,
        id: Uuid,
        page: filters::Page,
    ) -> RpcResult<Vec<Editor>> {
        let (query, values) = queries::get_editors_by_book_id(id, page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Editor>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|editor| editor.into())
                .collect(),
        )
    }

    async fn get_copy_by_id(self, _: Context, id: Uuid) -> RpcResult<Copy> {
        let (query, values) = queries::get_copy_by_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Copy>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_copies(self, _: Context, page: filters::Page) -> RpcResult<Vec<Copy>> {
        let (query, values) = queries::get_copies(page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Copy>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|copy| copy.into())
                .collect(),
        )
    }

    async fn get_copies_by_book_id(
        self,
        _: Context,
        id: Uuid,
        page: filters::Page,
    ) -> RpcResult<Vec<Copy>> {
        let (query, values) = queries::get_copies_by_book_id(id, page).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Copy>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|language| language.into())
                .collect(),
        )
    }

    async fn get_book_by_id(self, _: Context, id: Uuid) -> RpcResult<Book> {
        let (query, values) = queries::get_book_by_id(id).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Book>(&query), &values)
                .fetch_one(&mut self.get_db_connection().await)
                .await?
                .into(),
        )
    }

    async fn get_books(
        self,
        _: Context,
        page: filters::Page,
        book: filters::Book,
    ) -> RpcResult<Vec<Book>> {
        let (query, values) = queries::get_books(page, book).build(PostgresQueryBuilder);

        Ok(
            bind_query_as(query_as::<_, db_models::Book>(&query), &values)
                .fetch_all(&mut self.get_db_connection().await)
                .await?
                .into_iter()
                .map(|book| book.into())
                .collect(),
        )
    }
}
