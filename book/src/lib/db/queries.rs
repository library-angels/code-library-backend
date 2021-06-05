use sea_query::{Alias, Cond, Expr, Order, Query, SelectStatement};
use uuid::Uuid;

use super::schema;
use helpers::filters;

pub(crate) fn get_language_by_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Languages::Id,
            schema::Languages::IsoCode,
            schema::Languages::Name,
        ])
        .from(schema::Languages::Table)
        .and_where(Expr::col(schema::Languages::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_language_by_book_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![
            (schema::Languages::Table, schema::Languages::Id),
            (schema::Languages::Table, schema::Languages::IsoCode),
            (schema::Languages::Table, schema::Languages::Name),
        ])
        .from(schema::Languages::Table)
        .inner_join(
            schema::Books::Table,
            Expr::tbl(schema::Languages::Table, schema::Books::Id)
                .equals(schema::Books::Table, schema::Books::LanguageId),
        )
        .and_where(Expr::tbl(schema::Books::Table, schema::Books::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_languages(page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Languages::Id,
            schema::Languages::IsoCode,
            schema::Languages::Name,
        ])
        .from_subquery(
            Query::select()
                .columns(vec![
                    schema::Languages::Id,
                    schema::Languages::IsoCode,
                    schema::Languages::Name,
                ])
                .from(schema::Languages::Table)
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => Expr::col(schema::Languages::Id).gt(id),
                    filters::Cursor::Before(id) => Expr::col(schema::Languages::Id).lt(id),
                })
                .order_by(
                    schema::Languages::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Languages::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_category_by_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Categories::Id, schema::Categories::Name])
        .from(schema::Categories::Table)
        .and_where(Expr::col(schema::Categories::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_category_by_book_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![
            (schema::Categories::Table, schema::Categories::Id),
            (schema::Categories::Table, schema::Categories::Name),
        ])
        .from(schema::Categories::Table)
        .inner_join(
            schema::Books::Table,
            Expr::tbl(schema::Categories::Table, schema::Books::Id)
                .equals(schema::Books::Table, schema::Books::CategoryId),
        )
        .and_where(Expr::tbl(schema::Books::Table, schema::Books::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_categories(page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Categories::Id, schema::Categories::Name])
        .from_subquery(
            Query::select()
                .columns(vec![schema::Categories::Id, schema::Categories::Name])
                .from(schema::Categories::Table)
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => Expr::col(schema::Categories::Id).gt(id),
                    filters::Cursor::Before(id) => Expr::col(schema::Categories::Id).lt(id),
                })
                .order_by(
                    schema::Categories::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Categories::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_publisher_by_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Publishers::Id, schema::Publishers::Name])
        .from(schema::Publishers::Table)
        .and_where(Expr::col(schema::Publishers::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_publisher_by_book_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![
            (schema::Publishers::Table, schema::Publishers::Id),
            (schema::Publishers::Table, schema::Publishers::Name),
        ])
        .from(schema::Publishers::Table)
        .inner_join(
            schema::Books::Table,
            Expr::tbl(schema::Publishers::Table, schema::Books::Id)
                .equals(schema::Books::Table, schema::Books::PublisherId),
        )
        .and_where(Expr::tbl(schema::Books::Table, schema::Books::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_publishers(page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Publishers::Id, schema::Publishers::Name])
        .from_subquery(
            Query::select()
                .columns(vec![schema::Publishers::Id, schema::Publishers::Name])
                .from(schema::Publishers::Table)
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => Expr::col(schema::Publishers::Id).gt(id),
                    filters::Cursor::Before(id) => Expr::col(schema::Publishers::Id).lt(id),
                })
                .order_by(
                    schema::Publishers::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Publishers::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_series_by_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Series::Id,
            schema::Series::PublisherId,
            schema::Series::Name,
        ])
        .from(schema::Series::Table)
        .and_where(Expr::col(schema::Series::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_series_by_book_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![
            (schema::Series::Table, schema::Series::Id),
            (schema::Series::Table, schema::Series::PublisherId),
            (schema::Series::Table, schema::Series::Name),
        ])
        .from(schema::Series::Table)
        .inner_join(
            schema::Books::Table,
            Expr::tbl(schema::Series::Table, schema::Books::Id)
                .equals(schema::Books::Table, schema::Books::SeriesId),
        )
        .and_where(Expr::tbl(schema::Books::Table, schema::Books::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_series(page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Series::Id,
            schema::Series::PublisherId,
            schema::Series::Name,
        ])
        .from_subquery(
            Query::select()
                .columns(vec![
                    schema::Series::Id,
                    schema::Series::PublisherId,
                    schema::Series::Name,
                ])
                .from(schema::Series::Table)
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => Expr::col(schema::Series::Id).gt(id),
                    filters::Cursor::Before(id) => Expr::col(schema::Series::Id).lt(id),
                })
                .order_by(
                    schema::Series::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Series::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_subject_area_by_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![schema::SubjectAreas::Id, schema::SubjectAreas::Name])
        .from(schema::SubjectAreas::Table)
        .and_where(Expr::col(schema::SubjectAreas::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_subject_areas(page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![schema::SubjectAreas::Id, schema::SubjectAreas::Name])
        .from_subquery(
            Query::select()
                .columns(vec![schema::SubjectAreas::Id, schema::SubjectAreas::Name])
                .from(schema::SubjectAreas::Table)
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => Expr::col(schema::SubjectAreas::Id).gt(id),
                    filters::Cursor::Before(id) => Expr::col(schema::SubjectAreas::Id).lt(id),
                })
                .order_by(
                    schema::SubjectAreas::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::SubjectAreas::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_subject_areas_by_book_id(id: Uuid, page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![schema::SubjectAreas::Id, schema::SubjectAreas::Name])
        .from_subquery(
            Query::select()
                .columns(vec![
                    (schema::SubjectAreas::Table, schema::SubjectAreas::Id),
                    (schema::SubjectAreas::Table, schema::SubjectAreas::Name),
                ])
                .from(schema::BooksSubjectAreas::Table)
                .inner_join(
                    schema::SubjectAreas::Table,
                    Expr::col(schema::BooksSubjectAreas::SubjectAreaId)
                        .equals(schema::SubjectAreas::Table, schema::SubjectAreas::Id),
                )
                .and_where(Expr::col(schema::BooksSubjectAreas::BookId).eq(id))
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => {
                        Expr::tbl(schema::SubjectAreas::Table, schema::SubjectAreas::Id).gt(id)
                    }
                    filters::Cursor::Before(id) => {
                        Expr::tbl(schema::SubjectAreas::Table, schema::SubjectAreas::Id).lt(id)
                    }
                })
                .order_by(
                    schema::SubjectAreas::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::SubjectAreas::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_tag_by_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Tags::Id, schema::Tags::Name])
        .from(schema::Tags::Table)
        .and_where(Expr::col(schema::Tags::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_tags(page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Tags::Id, schema::Tags::Name])
        .from_subquery(
            Query::select()
                .columns(vec![schema::Tags::Id, schema::Tags::Name])
                .from(schema::Tags::Table)
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => Expr::col(schema::Tags::Id).gt(id),
                    filters::Cursor::Before(id) => Expr::col(schema::Tags::Id).lt(id),
                })
                .order_by(
                    schema::Tags::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Tags::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_tags_by_book_id(id: Uuid, page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Tags::Id, schema::Tags::Name])
        .from_subquery(
            Query::select()
                .columns(vec![
                    (schema::Tags::Table, schema::Tags::Id),
                    (schema::Tags::Table, schema::Tags::Name),
                ])
                .from(schema::BooksTags::Table)
                .inner_join(
                    schema::Tags::Table,
                    Expr::col(schema::BooksTags::TagId)
                        .equals(schema::Tags::Table, schema::Tags::Id),
                )
                .and_where(Expr::col(schema::BooksTags::BookId).eq(id))
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => {
                        Expr::tbl(schema::Tags::Table, schema::Tags::Id).gt(id)
                    }
                    filters::Cursor::Before(id) => {
                        Expr::tbl(schema::Tags::Table, schema::Tags::Id).lt(id)
                    }
                })
                .order_by(
                    schema::Tags::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Tags::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_author_by_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![
            (schema::Persons::Table, schema::Persons::Id),
            (schema::Persons::Table, schema::Persons::FirstName),
            (schema::Persons::Table, schema::Persons::LastName),
            (schema::Persons::Table, schema::Persons::DateOfBirth),
            (schema::Persons::Table, schema::Persons::Isni),
            (schema::Persons::Table, schema::Persons::Orcid),
            (schema::Persons::Table, schema::Persons::Oclc),
        ])
        .from(schema::BooksAuthors::Table)
        .inner_join(
            schema::Persons::Table,
            Expr::tbl(schema::BooksAuthors::Table, schema::BooksAuthors::PersonId)
                .equals(schema::Persons::Table, schema::Persons::Id),
        )
        .and_where(Expr::tbl(schema::Persons::Table, schema::Persons::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_authors(page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Persons::Id,
            schema::Persons::FirstName,
            schema::Persons::LastName,
            schema::Persons::DateOfBirth,
            schema::Persons::Isni,
            schema::Persons::Orcid,
            schema::Persons::Oclc,
        ])
        .from_subquery(
            Query::select()
                .columns(vec![
                    (schema::Persons::Table, schema::Persons::Id),
                    (schema::Persons::Table, schema::Persons::FirstName),
                    (schema::Persons::Table, schema::Persons::LastName),
                    (schema::Persons::Table, schema::Persons::DateOfBirth),
                    (schema::Persons::Table, schema::Persons::Isni),
                    (schema::Persons::Table, schema::Persons::Orcid),
                    (schema::Persons::Table, schema::Persons::Oclc),
                ])
                .from(schema::BooksAuthors::Table)
                .inner_join(
                    schema::Persons::Table,
                    Expr::tbl(schema::Persons::Table, schema::Persons::Id)
                        .equals(schema::BooksAuthors::Table, schema::BooksAuthors::PersonId),
                )
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => {
                        Expr::tbl(schema::Persons::Table, schema::Persons::Id).gt(id)
                    }
                    filters::Cursor::Before(id) => {
                        Expr::tbl(schema::Persons::Table, schema::Persons::Id).lt(id)
                    }
                })
                .order_by(
                    schema::Persons::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Persons::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_authors_by_book_id(id: Uuid, page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Persons::Id,
            schema::Persons::FirstName,
            schema::Persons::LastName,
            schema::Persons::DateOfBirth,
            schema::Persons::Isni,
            schema::Persons::Orcid,
            schema::Persons::Oclc,
        ])
        .from_subquery(
            Query::select()
                .columns(vec![
                    (schema::Persons::Table, schema::Persons::Id),
                    (schema::Persons::Table, schema::Persons::FirstName),
                    (schema::Persons::Table, schema::Persons::LastName),
                    (schema::Persons::Table, schema::Persons::DateOfBirth),
                    (schema::Persons::Table, schema::Persons::Isni),
                    (schema::Persons::Table, schema::Persons::Orcid),
                    (schema::Persons::Table, schema::Persons::Oclc),
                ])
                .from(schema::BooksAuthors::Table)
                .inner_join(
                    schema::Persons::Table,
                    Expr::col(schema::BooksAuthors::PersonId)
                        .equals(schema::Persons::Table, schema::Persons::Id),
                )
                .inner_join(
                    schema::Books::Table,
                    Expr::col(schema::BooksAuthors::BookId)
                        .equals(schema::Books::Table, schema::Books::Id),
                )
                .and_where(Expr::tbl(schema::Books::Table, schema::Books::Id).eq(id))
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => {
                        Expr::tbl(schema::Persons::Table, schema::Persons::Id).gt(id)
                    }
                    filters::Cursor::Before(id) => {
                        Expr::tbl(schema::Persons::Table, schema::Persons::Id).lt(id)
                    }
                })
                .order_by(
                    schema::Books::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Persons::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_editor_by_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![
            (schema::Persons::Table, schema::Persons::Id),
            (schema::Persons::Table, schema::Persons::FirstName),
            (schema::Persons::Table, schema::Persons::LastName),
            (schema::Persons::Table, schema::Persons::DateOfBirth),
            (schema::Persons::Table, schema::Persons::Isni),
            (schema::Persons::Table, schema::Persons::Orcid),
            (schema::Persons::Table, schema::Persons::Oclc),
        ])
        .from(schema::BooksEditors::Table)
        .inner_join(
            schema::Persons::Table,
            Expr::tbl(schema::BooksEditors::Table, schema::BooksEditors::PersonId)
                .equals(schema::Persons::Table, schema::Persons::Id),
        )
        .and_where(Expr::tbl(schema::Persons::Table, schema::Persons::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_editors(page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Persons::Id,
            schema::Persons::FirstName,
            schema::Persons::LastName,
            schema::Persons::DateOfBirth,
            schema::Persons::Isni,
            schema::Persons::Orcid,
            schema::Persons::Oclc,
        ])
        .from_subquery(
            Query::select()
                .columns(vec![
                    (schema::Persons::Table, schema::Persons::Id),
                    (schema::Persons::Table, schema::Persons::FirstName),
                    (schema::Persons::Table, schema::Persons::LastName),
                    (schema::Persons::Table, schema::Persons::DateOfBirth),
                    (schema::Persons::Table, schema::Persons::Isni),
                    (schema::Persons::Table, schema::Persons::Orcid),
                    (schema::Persons::Table, schema::Persons::Oclc),
                ])
                .from(schema::BooksEditors::Table)
                .inner_join(
                    schema::Persons::Table,
                    Expr::tbl(schema::Persons::Table, schema::Persons::Id)
                        .equals(schema::BooksEditors::Table, schema::BooksEditors::PersonId),
                )
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => {
                        Expr::tbl(schema::Persons::Table, schema::Persons::Id).gt(id)
                    }
                    filters::Cursor::Before(id) => {
                        Expr::tbl(schema::Persons::Table, schema::Persons::Id).lt(id)
                    }
                })
                .order_by(
                    schema::Persons::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Persons::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_editors_by_book_id(id: Uuid, page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Persons::Id,
            schema::Persons::FirstName,
            schema::Persons::LastName,
            schema::Persons::DateOfBirth,
            schema::Persons::Isni,
            schema::Persons::Orcid,
            schema::Persons::Oclc,
        ])
        .from_subquery(
            Query::select()
                .columns(vec![
                    (schema::Persons::Table, schema::Persons::Id),
                    (schema::Persons::Table, schema::Persons::FirstName),
                    (schema::Persons::Table, schema::Persons::LastName),
                    (schema::Persons::Table, schema::Persons::DateOfBirth),
                    (schema::Persons::Table, schema::Persons::Isni),
                    (schema::Persons::Table, schema::Persons::Orcid),
                    (schema::Persons::Table, schema::Persons::Oclc),
                ])
                .from(schema::BooksEditors::Table)
                .inner_join(
                    schema::Persons::Table,
                    Expr::col(schema::BooksEditors::PersonId)
                        .equals(schema::Persons::Table, schema::Persons::Id),
                )
                .inner_join(
                    schema::Books::Table,
                    Expr::col(schema::BooksEditors::BookId)
                        .equals(schema::Books::Table, schema::Books::Id),
                )
                .and_where(Expr::tbl(schema::Books::Table, schema::Books::Id).eq(id))
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => {
                        Expr::tbl(schema::Persons::Table, schema::Persons::Id).gt(id)
                    }
                    filters::Cursor::Before(id) => {
                        Expr::tbl(schema::Persons::Table, schema::Persons::Id).lt(id)
                    }
                })
                .order_by(
                    schema::Books::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Persons::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_copy_by_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Copies::Id,
            schema::Copies::BookId,
            schema::Copies::CodeIdentifierCopyId,
            schema::Copies::CreatedAt,
            schema::Copies::CreatedBy,
        ])
        .from(schema::Copies::Table)
        .and_where(Expr::col(schema::Copies::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_copies(page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Copies::Id,
            schema::Copies::BookId,
            schema::Copies::CodeIdentifierCopyId,
            schema::Copies::CreatedAt,
            schema::Copies::CreatedBy,
        ])
        .from_subquery(
            Query::select()
                .columns(vec![
                    schema::Copies::Id,
                    schema::Copies::BookId,
                    schema::Copies::CodeIdentifierCopyId,
                    schema::Copies::CreatedAt,
                    schema::Copies::CreatedBy,
                ])
                .from(schema::Copies::Table)
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => Expr::col(schema::Copies::Id).gt(id),
                    filters::Cursor::Before(id) => Expr::col(schema::Copies::Id).lt(id),
                })
                .order_by(
                    schema::Copies::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Copies::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_copies_by_book_id(id: Uuid, page: filters::Page) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Copies::Id,
            schema::Copies::BookId,
            schema::Copies::CodeIdentifierCopyId,
            schema::Copies::CreatedAt,
            schema::Copies::CreatedBy,
        ])
        .from_subquery(
            Query::select()
                .columns(vec![
                    (schema::Copies::Table, schema::Copies::Id),
                    (schema::Copies::Table, schema::Copies::BookId),
                    (schema::Copies::Table, schema::Copies::CodeIdentifierCopyId),
                    (schema::Copies::Table, schema::Copies::CreatedAt),
                    (schema::Copies::Table, schema::Copies::CreatedBy),
                ])
                .from(schema::Books::Table)
                .inner_join(
                    schema::Copies::Table,
                    Expr::tbl(schema::Books::Table, schema::Books::Id)
                        .equals(schema::Copies::Table, schema::Copies::BookId),
                )
                .and_where(Expr::tbl(schema::Books::Table, schema::Books::Id).eq(id))
                .and_where(match page.get_cursor() {
                    filters::Cursor::After(id) => {
                        Expr::tbl(schema::Copies::Table, schema::Copies::Id).gt(id)
                    }
                    filters::Cursor::Before(id) => {
                        Expr::tbl(schema::Copies::Table, schema::Copies::Id).lt(id)
                    }
                })
                .order_by(
                    schema::Copies::Id,
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Copies::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_book_by_id(id: Uuid) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Books::Id,
            schema::Books::CodeIdentifier,
            schema::Books::Isbn,
            schema::Books::Issn,
            schema::Books::ReleaseYear,
            schema::Books::Edition,
            schema::Books::Pages,
            schema::Books::Title,
            schema::Books::Subtitle,
            schema::Books::Description,
        ])
        .from(schema::Books::Table)
        .and_where(Expr::col(schema::Books::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_books(page: filters::Page, book: filters::Book) -> SelectStatement {
    Query::select()
        .columns(vec![
            schema::Books::Id,
            schema::Books::CodeIdentifier,
            schema::Books::Isbn,
            schema::Books::Issn,
            schema::Books::ReleaseYear,
            schema::Books::Edition,
            schema::Books::Pages,
            schema::Books::Subtitle,
            schema::Books::Title,
            schema::Books::Description,
        ])
        .from_subquery(
            Query::select()
                .columns(vec![
                    (schema::Books::Table, schema::Books::Id),
                    (schema::Books::Table, schema::Books::CodeIdentifier),
                    (schema::Books::Table, schema::Books::Isbn),
                    (schema::Books::Table, schema::Books::Issn),
                    (schema::Books::Table, schema::Books::ReleaseYear),
                    (schema::Books::Table, schema::Books::Edition),
                    (schema::Books::Table, schema::Books::Pages),
                    (schema::Books::Table, schema::Books::Subtitle),
                    (schema::Books::Table, schema::Books::Title),
                    (schema::Books::Table, schema::Books::Description),
                ])
                .from(schema::Books::Table)
                .inner_join(
                    schema::Categories::Table,
                    Expr::tbl(schema::Books::Table, schema::Books::CategoryId)
                        .equals(schema::Categories::Table, schema::Categories::Id),
                )
                .inner_join(
                    schema::Publishers::Table,
                    Expr::tbl(schema::Books::Table, schema::Books::PublisherId)
                        .equals(schema::Publishers::Table, schema::Publishers::Id),
                )
                .left_join(
                    schema::Series::Table,
                    Expr::tbl(schema::Books::Table, schema::Books::SeriesId)
                        .equals(schema::Series::Table, schema::Series::Id),
                )
                .left_join(
                    schema::BooksTags::Table,
                    Expr::tbl(schema::Books::Table, schema::Books::Id)
                        .equals(schema::BooksTags::Table, schema::BooksTags::BookId),
                )
                .left_join(
                    schema::Tags::Table,
                    Expr::tbl(schema::BooksTags::Table, schema::BooksTags::TagId)
                        .equals(schema::Tags::Table, schema::Tags::Id),
                )
                .cond_where(
                    Cond::all()
                        .add_option(book.get_categories().map(|categories| {
                            Cond::all().add(
                                Expr::tbl(schema::Categories::Table, schema::Categories::Name)
                                    .is_in(categories),
                            )
                        }))
                        .add_option(book.get_publishers().map(|publishers| {
                            Cond::all().add(
                                Expr::tbl(schema::Publishers::Table, schema::Publishers::Name)
                                    .is_in(publishers),
                            )
                        }))
                        .add_option(book.get_series().map(|series| {
                            Cond::all().add(
                                Expr::tbl(schema::Series::Table, schema::Series::Name)
                                    .is_in(series),
                            )
                        }))
                        .add_option(book.get_tags().map(|tags| {
                            Cond::all()
                                .add(Expr::tbl(schema::Tags::Table, schema::Tags::Name).is_in(tags))
                        }))
                        .add(Cond::all().add(match page.get_cursor() {
                            filters::Cursor::After(id) => {
                                Expr::tbl(schema::Books::Table, schema::Books::Id).gt(id)
                            }
                            filters::Cursor::Before(id) => {
                                Expr::tbl(schema::Books::Table, schema::Books::Id).lt(id)
                            }
                        })),
                )
                .order_by(
                    (schema::Books::Table, schema::Books::Id),
                    match page.get_cursor() {
                        filters::Cursor::After(_) => Order::Asc,
                        filters::Cursor::Before(_) => Order::Desc,
                    },
                )
                .limit(page.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Books::Id, Order::Asc)
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_query::PostgresQueryBuilder;

    #[test]
    fn ut_get_language() {
        let id = Uuid::new_v4();
        let query = get_language_by_id(id);

        assert_eq!(
            format!(
                r#"SELECT "id", "iso_code", "name" FROM "languages" WHERE "id" = '{}'"#,
                id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_language_by_book_id() {
        let book_id = Uuid::new_v4();
        let query = get_language_by_book_id(book_id);

        assert_eq!(
            format!(
                r#"SELECT "languages"."id", "languages"."iso_code", "languages"."name" FROM "languages" INNER JOIN "books" ON "languages"."id" = "books"."language_id" WHERE "books"."id" = '{}'"#,
                book_id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_languages_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_languages(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "iso_code", "name" FROM (SELECT "id", "iso_code", "name" FROM "languages" WHERE "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_languages_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_languages(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "iso_code", "name" FROM (SELECT "id", "iso_code", "name" FROM "languages" WHERE "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_category() {
        let id = Uuid::new_v4();
        let query = get_category_by_id(id);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM "categories" WHERE "id" = '{}'"#,
                id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_category_by_book_id() {
        let book_id = Uuid::new_v4();
        let query = get_category_by_book_id(book_id);

        assert_eq!(
            format!(
                r#"SELECT "categories"."id", "categories"."name" FROM "categories" INNER JOIN "books" ON "categories"."id" = "books"."category_id" WHERE "books"."id" = '{}'"#,
                book_id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_categories_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_categories(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "categories" WHERE "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_categories_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_categories(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "categories" WHERE "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_publisher() {
        let id = Uuid::new_v4();
        let query = get_publisher_by_id(id);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM "publishers" WHERE "id" = '{}'"#,
                id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_publisher_by_book_id() {
        let book_id = Uuid::new_v4();
        let query = get_publisher_by_book_id(book_id);

        assert_eq!(
            format!(
                r#"SELECT "publishers"."id", "publishers"."name" FROM "publishers" INNER JOIN "books" ON "publishers"."id" = "books"."publisher_id" WHERE "books"."id" = '{}'"#,
                book_id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_publishers_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_publishers(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "publishers" WHERE "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_publishers_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_publishers(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "publishers" WHERE "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_series() {
        let id = Uuid::new_v4();
        let query = get_series_by_id(id);

        assert_eq!(
            format!(
                r#"SELECT "id", "publisher_id", "name" FROM "series" WHERE "id" = '{}'"#,
                id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_series_by_book_id() {
        let book_id = Uuid::new_v4();
        let query = get_series_by_book_id(book_id);

        assert_eq!(
            format!(
                r#"SELECT "series"."id", "series"."publisher_id", "series"."name" FROM "series" INNER JOIN "books" ON "series"."id" = "books"."series_id" WHERE "books"."id" = '{}'"#,
                book_id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_series_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_series(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "publisher_id", "name" FROM (SELECT "id", "publisher_id", "name" FROM "series" WHERE "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_series_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_series(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "publisher_id", "name" FROM (SELECT "id", "publisher_id", "name" FROM "series" WHERE "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_subject_area() {
        let id = Uuid::new_v4();
        let query = get_subject_area_by_id(id);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM "subject_areas" WHERE "id" = '{}'"#,
                id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_subject_areas_by_book_id_page_after() {
        let book_id = Uuid::new_v4();
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_subject_areas_by_book_id(book_id, page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "subject_areas"."id", "subject_areas"."name" FROM "books_subject_areas" INNER JOIN "subject_areas" ON "subject_area_id" = "subject_areas"."id" WHERE "book_id" = '{}' AND "subject_areas"."id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_subject_areas_by_book_id_page_before() {
        let book_id = Uuid::new_v4();
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_subject_areas_by_book_id(book_id, page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "subject_areas"."id", "subject_areas"."name" FROM "books_subject_areas" INNER JOIN "subject_areas" ON "subject_area_id" = "subject_areas"."id" WHERE "book_id" = '{}' AND "subject_areas"."id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_subject_areas_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_subject_areas(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "subject_areas" WHERE "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_subject_areas_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_subject_areas(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "subject_areas" WHERE "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_tag() {
        let id = Uuid::new_v4();
        let query = get_tag_by_id(id);

        assert_eq!(
            format!(r#"SELECT "id", "name" FROM "tags" WHERE "id" = '{}'"#, id),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_tags_by_book_id_page_after() {
        let book_id = Uuid::new_v4();
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_tags_by_book_id(book_id, page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "tags"."id", "tags"."name" FROM "books_tags" INNER JOIN "tags" ON "tag_id" = "tags"."id" WHERE "book_id" = '{}' AND "tags"."id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_tags_by_book_id_page_before() {
        let book_id = Uuid::new_v4();
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_tags_by_book_id(book_id, page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "tags"."id", "tags"."name" FROM "books_tags" INNER JOIN "tags" ON "tag_id" = "tags"."id" WHERE "book_id" = '{}' AND "tags"."id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_tags_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_tags(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "tags" WHERE "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_tags_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_tags(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "tags" WHERE "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_author() {
        let id = Uuid::new_v4();
        let query = get_author_by_id(id);

        assert_eq!(
            format!(
                r#"SELECT "persons"."id", "persons"."first_name", "persons"."last_name", "persons"."date_of_birth", "persons"."isni", "persons"."orcid", "persons"."oclc" FROM "books_authors" INNER JOIN "persons" ON "books_authors"."person_id" = "persons"."id" WHERE "persons"."id" = '{}'"#,
                id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_authors_by_book_id_page_after() {
        let book_id = Uuid::new_v4();
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_authors_by_book_id(book_id, page);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "persons"."id", "persons"."first_name", "persons"."last_name", "persons"."date_of_birth", "persons"."isni", "persons"."orcid", "persons"."oclc" FROM "books_authors" INNER JOIN "persons" ON "person_id" = "persons"."id" INNER JOIN "books" ON "book_id" = "books"."id" WHERE "books"."id" = '{}' AND "persons"."id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_authors_by_book_id_page_before() {
        let book_id = Uuid::new_v4();
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_authors_by_book_id(book_id, page);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "persons"."id", "persons"."first_name", "persons"."last_name", "persons"."date_of_birth", "persons"."isni", "persons"."orcid", "persons"."oclc" FROM "books_authors" INNER JOIN "persons" ON "person_id" = "persons"."id" INNER JOIN "books" ON "book_id" = "books"."id" WHERE "books"."id" = '{}' AND "persons"."id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_authors_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_authors(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "persons"."id", "persons"."first_name", "persons"."last_name", "persons"."date_of_birth", "persons"."isni", "persons"."orcid", "persons"."oclc" FROM "books_authors" INNER JOIN "persons" ON "persons"."id" = "books_authors"."person_id" WHERE "persons"."id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_authors_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_authors(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "persons"."id", "persons"."first_name", "persons"."last_name", "persons"."date_of_birth", "persons"."isni", "persons"."orcid", "persons"."oclc" FROM "books_authors" INNER JOIN "persons" ON "persons"."id" = "books_authors"."person_id" WHERE "persons"."id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_editor() {
        let id = Uuid::new_v4();
        let query = get_editor_by_id(id);

        assert_eq!(
            format!(
                r#"SELECT "persons"."id", "persons"."first_name", "persons"."last_name", "persons"."date_of_birth", "persons"."isni", "persons"."orcid", "persons"."oclc" FROM "books_editors" INNER JOIN "persons" ON "books_editors"."person_id" = "persons"."id" WHERE "persons"."id" = '{}'"#,
                id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_editors_by_book_id_page_after() {
        let book_id = Uuid::new_v4();
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_editors_by_book_id(book_id, page);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "persons"."id", "persons"."first_name", "persons"."last_name", "persons"."date_of_birth", "persons"."isni", "persons"."orcid", "persons"."oclc" FROM "books_editors" INNER JOIN "persons" ON "person_id" = "persons"."id" INNER JOIN "books" ON "book_id" = "books"."id" WHERE "books"."id" = '{}' AND "persons"."id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_editors_by_book_id_page_before() {
        let book_id = Uuid::new_v4();
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_editors_by_book_id(book_id, page);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "persons"."id", "persons"."first_name", "persons"."last_name", "persons"."date_of_birth", "persons"."isni", "persons"."orcid", "persons"."oclc" FROM "books_editors" INNER JOIN "persons" ON "person_id" = "persons"."id" INNER JOIN "books" ON "book_id" = "books"."id" WHERE "books"."id" = '{}' AND "persons"."id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_editors_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_editors(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "persons"."id", "persons"."first_name", "persons"."last_name", "persons"."date_of_birth", "persons"."isni", "persons"."orcid", "persons"."oclc" FROM "books_editors" INNER JOIN "persons" ON "persons"."id" = "books_editors"."person_id" WHERE "persons"."id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_editors_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_editors(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "persons"."id", "persons"."first_name", "persons"."last_name", "persons"."date_of_birth", "persons"."isni", "persons"."orcid", "persons"."oclc" FROM "books_editors" INNER JOIN "persons" ON "persons"."id" = "books_editors"."person_id" WHERE "persons"."id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_copy() {
        let id = Uuid::new_v4();
        let query = get_copy_by_id(id);

        assert_eq!(
            format!(
                r#"SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM "copies" WHERE "id" = '{}'"#,
                id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_copy_by_book_id_page_after() {
        let book_id = Uuid::new_v4();
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_copies_by_book_id(book_id, page);

        assert_eq!(
            format!(
                r#"SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM (SELECT "copies"."id", "copies"."book_id", "copies"."code_identifier_copy_id", "copies"."created_at", "copies"."created_by" FROM "books" INNER JOIN "copies" ON "books"."id" = "copies"."book_id" WHERE "books"."id" = '{}' AND "copies"."id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_copy_by_book_id_page_before() {
        let book_id = Uuid::new_v4();
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_copies_by_book_id(book_id, page);

        assert_eq!(
            format!(
                r#"SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM (SELECT "copies"."id", "copies"."book_id", "copies"."code_identifier_copy_id", "copies"."created_at", "copies"."created_by" FROM "books" INNER JOIN "copies" ON "books"."id" = "copies"."book_id" WHERE "books"."id" = '{}' AND "copies"."id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_copies_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let query = get_copies(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM (SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM "copies" WHERE "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_copies_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let query = get_copies(page);

        assert_eq!(
            format!(
                r#"SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM (SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM "copies" WHERE "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_book() {
        let id = Uuid::new_v4();
        let query = get_book_by_id(id);

        assert_eq!(
            format!(
                r#"SELECT "id", "code_identifier", "isbn", "issn", "release_year", "edition", "pages", "title", "subtitle", "description" FROM "books" WHERE "id" = '{}'"#,
                id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_books_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::After(filter_id),
            filters::Items::new(items),
        );
        let book = filters::Book::new(None, None, None, None);
        let query = get_books(page, book);

        assert_eq!(
            format!(
                r#"SELECT "id", "code_identifier", "isbn", "issn", "release_year", "edition", "pages", "subtitle", "title", "description" FROM (SELECT "books"."id", "books"."code_identifier", "books"."isbn", "books"."issn", "books"."release_year", "books"."edition", "books"."pages", "books"."subtitle", "books"."title", "books"."description" FROM "books" INNER JOIN "categories" ON "books"."category_id" = "categories"."id" INNER JOIN "publishers" ON "books"."publisher_id" = "publishers"."id" LEFT JOIN "series" ON "books"."series_id" = "series"."id" LEFT JOIN "books_tags" ON "books"."id" = "books_tags"."book_id" LEFT JOIN "tags" ON "books_tags"."tag_id" = "tags"."id" WHERE "books"."id" > '{}' ORDER BY "books"."id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_books_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let book = filters::Book::new(None, None, None, None);
        let query = get_books(page, book);

        assert_eq!(
            format!(
                r#"SELECT "id", "code_identifier", "isbn", "issn", "release_year", "edition", "pages", "subtitle", "title", "description" FROM (SELECT "books"."id", "books"."code_identifier", "books"."isbn", "books"."issn", "books"."release_year", "books"."edition", "books"."pages", "books"."subtitle", "books"."title", "books"."description" FROM "books" INNER JOIN "categories" ON "books"."category_id" = "categories"."id" INNER JOIN "publishers" ON "books"."publisher_id" = "publishers"."id" LEFT JOIN "series" ON "books"."series_id" = "series"."id" LEFT JOIN "books_tags" ON "books"."id" = "books_tags"."book_id" LEFT JOIN "tags" ON "books_tags"."tag_id" = "tags"."id" WHERE "books"."id" < '{}' ORDER BY "books"."id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_books_filter_books() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page = filters::Page::new(
            filters::Cursor::Before(filter_id),
            filters::Items::new(items),
        );
        let book = filters::Book::new(
            Some(vec!["a".into()]),
            Some(vec!["b".into()]),
            Some(vec!["c".into()]),
            Some(vec!["d".into()]),
        );
        let query = get_books(page, book);

        assert_eq!(
            format!(
                r#"SELECT "id", "code_identifier", "isbn", "issn", "release_year", "edition", "pages", "subtitle", "title", "description" FROM (SELECT "books"."id", "books"."code_identifier", "books"."isbn", "books"."issn", "books"."release_year", "books"."edition", "books"."pages", "books"."subtitle", "books"."title", "books"."description" FROM "books" INNER JOIN "categories" ON "books"."category_id" = "categories"."id" INNER JOIN "publishers" ON "books"."publisher_id" = "publishers"."id" LEFT JOIN "series" ON "books"."series_id" = "series"."id" LEFT JOIN "books_tags" ON "books"."id" = "books_tags"."book_id" LEFT JOIN "tags" ON "books_tags"."tag_id" = "tags"."id" WHERE "categories"."name" IN ('a') AND "publishers"."name" IN ('b') AND "series"."name" IN ('c') AND "tags"."name" IN ('d') AND "books"."id" < '{}' ORDER BY "books"."id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }
}
