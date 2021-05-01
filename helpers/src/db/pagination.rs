use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::result::Error as DieselError;
use diesel::{pg::Pg, sql_types::BigInt, QueryId};
use std::{error::Error, fmt};

const DEFAULT_PER_PAGE: i64 = 10;

/// Provides pagination for database queries.
pub trait Paginate: Sized + Query {
    /// Returns a tuple `(Vec<T>, i64)` with the records of the requested page as the first and the amount of pages in the second element.
    ///
    /// Expects `page > 0`. Else `.load_and_count_pages()` will return `diesel::result::Error::QueryBuilderError(PaginatedError))`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate diesel;
    /// # use diesel::prelude::*;
    /// #
    /// # mod schema {
    /// #     table! {
    /// #         a (id) {
    /// #             id -> Int4,
    /// #         }
    /// #     }
    /// # }
    /// #
    /// # mod models {
    /// #     #[derive(Queryable)]
    /// #     pub struct A {
    /// #         id: i32
    /// #     }
    /// # }
    /// #
    /// # let tmp_conn = PgConnection::establish(&format!("postgres://postgres:password@{}", std::env::var("DB_HOST_SOCKET").unwrap())).unwrap();
    /// # diesel::sql_query("CREATE DATABASE helpers_db_pagination_doc_comment;").execute(&tmp_conn);
    /// # let conn = PgConnection::establish(&format!("postgres://postgres:password@{}/helpers_db_pagination_doc_comment", std::env::var("DB_HOST_SOCKET").unwrap())).unwrap();
    /// # diesel::sql_query("CREATE TABLE A(id SERIAL PRIMARY KEY);").execute(&conn);
    /// #
    /// use helpers::db::pagination::Paginate;
    /// use models::A;
    /// use schema::a;
    ///
    /// // items: Vec<A>, num_pages: i64
    /// let (items, num_pages) = a::table
    ///     .into_boxed()
    ///     .paginate(1)  // page number (minimum: 1)
    ///     .per_page(10) // items per page (optional, default: 10, minimum: 1)
    ///     .load_and_count_pages::<A>(&conn)
    ///     .unwrap();
    ///
    /// # std::mem::drop(conn);
    /// # diesel::sql_query("DROP DATABASE helpers_db_pagination_doc_comment;").execute(&tmp_conn);
    /// ```
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T: Query> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: DEFAULT_PER_PAGE,
            page,
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
}

impl<T> Paginated<T> {
    /// Configure how many items one page should contain.
    ///
    /// Expects `per_page > 0`. Else `.load_and_count_pages()` will return `diesel::result::Error::QueryBuilderError(PaginatedError))`.
    pub fn per_page(mut self, per_page: i64) -> Self {
        self.per_page = per_page;
        self
    }

    pub fn load_and_count_pages<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        if self.page < 1 {
            let err = PaginatedError::PageIndexLessThanZero;
            Err(DieselError::QueryBuilderError(Box::new(err)))
        } else if self.per_page < 1 {
            let err = PaginatedError::PerPageLessThanZero;
            Err(DieselError::QueryBuilderError(Box::new(err)))
        } else {
            let per_page = self.per_page;
            let results = self.load::<(U, i64)>(conn)?;
            let total = results.get(0).map(|x| x.1).unwrap_or(0);
            let records = results.into_iter().map(|x| x.0).collect();
            let total_pages = (total as f64 / per_page as f64).ceil() as i64;
            Ok((records, total_pages))
        }
    }
}

#[derive(Debug)]
pub enum PaginatedError {
    PageIndexLessThanZero,
    PerPageLessThanZero,
}

impl Error for PaginatedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl fmt::Display for PaginatedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaginatedError::PageIndexLessThanZero => write!(f, "Page index less than zero."),
            PaginatedError::PerPageLessThanZero => write!(f, "Per page less than zero."),
        }
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    /* Implements the logic of the AST node.
     * Generated sql: SELECT *, COUNT(*) OVER () FROM (<passed-query>) t LIMIT $1 OFFSET $2;
     */
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}
