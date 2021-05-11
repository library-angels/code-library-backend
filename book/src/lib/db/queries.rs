use sea_query::{Alias, Expr, Order, Query, SelectStatement};
use uuid::Uuid;

use super::schema;
use helpers::types::{Page, PageFilter};

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
            schema::Languages::Id,
            schema::Languages::IsoCode,
            schema::Languages::Name,
        ])
        .from(schema::Languages::Table)
        .inner_join(
            schema::Books::Table,
            Expr::tbl(schema::Languages::Table, schema::Books::Id)
                .equals(schema::Books::Table, schema::Books::LanguageId),
        )
        .and_where(Expr::col(schema::Books::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_languages(page_filter: PageFilter) -> SelectStatement {
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
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Languages::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Languages::Id).lt(id),
                })
                .order_by(
                    schema::Languages::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
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
        .columns(vec![schema::Categories::Id, schema::Categories::Name])
        .from(schema::Categories::Table)
        .inner_join(
            schema::Books::Table,
            Expr::tbl(schema::Categories::Table, schema::Books::Id)
                .equals(schema::Books::Table, schema::Books::CategoryId),
        )
        .and_where(Expr::col(schema::Books::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_categories(page_filter: PageFilter) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Categories::Id, schema::Categories::Name])
        .from_subquery(
            Query::select()
                .columns(vec![schema::Categories::Id, schema::Categories::Name])
                .from(schema::Categories::Table)
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Categories::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Categories::Id).lt(id),
                })
                .order_by(
                    schema::Categories::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
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
        .columns(vec![schema::Publishers::Id, schema::Publishers::Name])
        .from(schema::Publishers::Table)
        .inner_join(
            schema::Books::Table,
            Expr::tbl(schema::Publishers::Table, schema::Books::Id)
                .equals(schema::Books::Table, schema::Books::PublisherId),
        )
        .and_where(Expr::col(schema::Books::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_publishers(page_filter: PageFilter) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Publishers::Id, schema::Publishers::Name])
        .from_subquery(
            Query::select()
                .columns(vec![schema::Publishers::Id, schema::Publishers::Name])
                .from(schema::Publishers::Table)
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Publishers::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Publishers::Id).lt(id),
                })
                .order_by(
                    schema::Publishers::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
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
            schema::Series::Id,
            schema::Series::PublisherId,
            schema::Series::Name,
        ])
        .from(schema::Series::Table)
        .inner_join(
            schema::Books::Table,
            Expr::tbl(schema::Series::Table, schema::Books::Id)
                .equals(schema::Books::Table, schema::Books::SeriesId),
        )
        .and_where(Expr::col(schema::Books::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_series(page_filter: PageFilter) -> SelectStatement {
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
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Series::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Series::Id).lt(id),
                })
                .order_by(
                    schema::Series::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
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

pub(crate) fn get_subject_areas(page_filter: PageFilter) -> SelectStatement {
    Query::select()
        .columns(vec![schema::SubjectAreas::Id, schema::SubjectAreas::Name])
        .from_subquery(
            Query::select()
                .columns(vec![schema::SubjectAreas::Id, schema::SubjectAreas::Name])
                .from(schema::SubjectAreas::Table)
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::SubjectAreas::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::SubjectAreas::Id).lt(id),
                })
                .order_by(
                    schema::SubjectAreas::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::SubjectAreas::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_subject_areas_by_book_id(id: Uuid, page_filter: PageFilter) -> SelectStatement {
    Query::select()
        .columns(vec![schema::SubjectAreas::Id, schema::SubjectAreas::Name])
        .from_subquery(
            Query::select()
                .columns(vec![schema::SubjectAreas::Id, schema::SubjectAreas::Name])
                .from(schema::BooksAuthors::Table)
                .inner_join(
                    schema::BooksSubjectAreas::Table,
                    Expr::col(schema::SubjectAreas::Id).equals(
                        schema::BooksSubjectAreas::Table,
                        schema::BooksSubjectAreas::SubjectAreaId,
                    ),
                )
                .and_where(Expr::col(schema::BooksSubjectAreas::BookId).eq(id))
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::SubjectAreas::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::SubjectAreas::Id).lt(id),
                })
                .order_by(
                    schema::SubjectAreas::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
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

pub(crate) fn get_tags(page_filter: PageFilter) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Tags::Id, schema::Tags::Name])
        .from_subquery(
            Query::select()
                .columns(vec![schema::Tags::Id, schema::Tags::Name])
                .from(schema::Tags::Table)
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Tags::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Tags::Id).lt(id),
                })
                .order_by(
                    schema::Tags::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Tags::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_tags_by_book_id(id: Uuid, page_filter: PageFilter) -> SelectStatement {
    Query::select()
        .columns(vec![schema::Tags::Id, schema::Tags::Name])
        .from_subquery(
            Query::select()
                .columns(vec![schema::Tags::Id, schema::Tags::Name])
                .from(schema::BooksTags::Table)
                .inner_join(
                    schema::Tags::Table,
                    Expr::col(schema::BooksTags::TagId)
                        .equals(schema::Tags::Table, schema::Tags::Id),
                )
                .and_where(Expr::col(schema::BooksTags::BookId).eq(id))
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Tags::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Tags::Id).lt(id),
                })
                .order_by(
                    schema::Tags::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Tags::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_author_by_id(id: Uuid) -> SelectStatement {
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
        .from(schema::BooksAuthors::Table)
        .inner_join(
            schema::Persons::Table,
            Expr::tbl(schema::BooksAuthors::Table, schema::BooksAuthors::PersonId)
                .equals(schema::Persons::Table, schema::Persons::Id),
        )
        .and_where(Expr::col(schema::Persons::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_authors(page_filter: PageFilter) -> SelectStatement {
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
                    schema::Persons::Id,
                    schema::Persons::FirstName,
                    schema::Persons::LastName,
                    schema::Persons::DateOfBirth,
                    schema::Persons::Isni,
                    schema::Persons::Orcid,
                    schema::Persons::Oclc,
                ])
                .from(schema::BooksAuthors::Table)
                .inner_join(
                    schema::Persons::Table,
                    Expr::col(schema::Persons::Id)
                        .equals(schema::BooksAuthors::Table, schema::BooksAuthors::PersonId),
                )
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Persons::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Persons::Id).lt(id),
                })
                .order_by(
                    schema::Persons::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Persons::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_authors_by_book_id(id: Uuid, page_filter: PageFilter) -> SelectStatement {
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
                    schema::Persons::Id,
                    schema::Persons::FirstName,
                    schema::Persons::LastName,
                    schema::Persons::DateOfBirth,
                    schema::Persons::Isni,
                    schema::Persons::Orcid,
                    schema::Persons::Oclc,
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
                .and_where(Expr::col(schema::Books::Id).eq(id))
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Persons::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Persons::Id).lt(id),
                })
                .order_by(
                    schema::Books::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64)
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Persons::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_editor_by_id(id: Uuid) -> SelectStatement {
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
        .from(schema::BooksEditors::Table)
        .inner_join(
            schema::Persons::Table,
            Expr::tbl(schema::BooksEditors::Table, schema::BooksEditors::PersonId)
                .equals(schema::Persons::Table, schema::Persons::Id),
        )
        .and_where(Expr::col(schema::Persons::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_editors(page_filter: PageFilter) -> SelectStatement {
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
                    schema::Persons::Id,
                    schema::Persons::FirstName,
                    schema::Persons::LastName,
                    schema::Persons::DateOfBirth,
                    schema::Persons::Isni,
                    schema::Persons::Orcid,
                    schema::Persons::Oclc,
                ])
                .from(schema::BooksEditors::Table)
                .inner_join(
                    schema::Persons::Table,
                    Expr::col(schema::Persons::Id)
                        .equals(schema::BooksEditors::Table, schema::BooksEditors::PersonId),
                )
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Persons::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Persons::Id).lt(id),
                })
                .order_by(
                    schema::Persons::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Persons::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_editors_by_book_id(id: Uuid, page_filter: PageFilter) -> SelectStatement {
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
                    schema::Persons::Id,
                    schema::Persons::FirstName,
                    schema::Persons::LastName,
                    schema::Persons::DateOfBirth,
                    schema::Persons::Isni,
                    schema::Persons::Orcid,
                    schema::Persons::Oclc,
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
                .and_where(Expr::col(schema::Books::Id).eq(id))
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Persons::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Persons::Id).lt(id),
                })
                .order_by(
                    schema::Books::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64)
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
        .and_where(Expr::col(schema::Copies::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_copies(page_filter: PageFilter) -> SelectStatement {
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
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Copies::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Copies::Id).lt(id),
                })
                .order_by(
                    schema::Copies::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
                .take(),
            Alias::new("t"),
        )
        .order_by(schema::Copies::Id, Order::Asc)
        .to_owned()
}

pub(crate) fn get_copies_by_book_id(id: Uuid, page_filter: PageFilter) -> SelectStatement {
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
                .inner_join(
                    schema::Copies::Table,
                    Expr::col(schema::Copies::BookId)
                        .equals(schema::Books::Table, schema::Books::Id),
                )
                .and_where(Expr::col(schema::Books::Id).eq(id))
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Copies::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Copies::Id).lt(id),
                })
                .order_by(
                    schema::Copies::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
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
        .and_where(Expr::col(schema::Books::Id).eq(id))
        .to_owned()
}

pub(crate) fn get_books(page_filter: PageFilter) -> SelectStatement {
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
                .from(schema::Books::Table)
                .and_where(match page_filter.get_page() {
                    Page::After(id) => Expr::col(schema::Books::Id).gt(id),
                    Page::Before(id) => Expr::col(schema::Books::Id).lt(id),
                })
                .order_by(
                    schema::Books::Id,
                    match page_filter.get_page() {
                        Page::After(_) => Order::Asc,
                        Page::Before(_) => Order::Desc,
                    },
                )
                .limit(page_filter.get_items() as u64) // should this not an i64
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
                r#"SELECT "id", "iso_code", "name" FROM "languages" INNER JOIN "books" ON "languages"."id" = "books"."language_id" WHERE "id" = '{}'"#,
                book_id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_languages_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_languages(page_filter);

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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_languages(page_filter);

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
                r#"SELECT "id", "name" FROM "categories" INNER JOIN "books" ON "categories"."id" = "books"."category_id" WHERE "id" = '{}'"#,
                book_id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_categories_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_categories(page_filter);

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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_categories(page_filter);

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
                r#"SELECT "id", "name" FROM "publishers" INNER JOIN "books" ON "publishers"."id" = "books"."publisher_id" WHERE "id" = '{}'"#,
                book_id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_publishers_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_publishers(page_filter);

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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_publishers(page_filter);

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
                r#"SELECT "id", "publisher_id", "name" FROM "series" INNER JOIN "books" ON "series"."id" = "books"."series_id" WHERE "id" = '{}'"#,
                book_id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_series_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_series(page_filter);

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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_series(page_filter);

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
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_subject_areas_by_book_id(book_id, page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "books_authors" INNER JOIN "books_subject_areas" ON "id" = "books_subject_areas"."subject_area_id" WHERE "book_id" = '{}' AND "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_subject_areas_by_book_id(book_id, page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "books_authors" INNER JOIN "books_subject_areas" ON "id" = "books_subject_areas"."subject_area_id" WHERE "book_id" = '{}' AND "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_subject_areas_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_subject_areas(page_filter);

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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_subject_areas(page_filter);

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
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_tags_by_book_id(book_id, page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "books_tags" INNER JOIN "tags" ON "tag_id" = "tags"."id" WHERE "book_id" = '{}' AND "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_tags_by_book_id(book_id, page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "name" FROM (SELECT "id", "name" FROM "books_tags" INNER JOIN "tags" ON "tag_id" = "tags"."id" WHERE "book_id" = '{}' AND "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_tags_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_tags(page_filter);

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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_tags(page_filter);

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
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM "books_authors" INNER JOIN "persons" ON "books_authors"."person_id" = "persons"."id" WHERE "id" = '{}'"#,
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
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_authors_by_book_id(book_id, page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM "books_authors" INNER JOIN "persons" ON "person_id" = "persons"."id" INNER JOIN "books" ON "book_id" = "books"."id" WHERE "id" = '{}' AND "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_authors_by_book_id(book_id, page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM "books_authors" INNER JOIN "persons" ON "person_id" = "persons"."id" INNER JOIN "books" ON "book_id" = "books"."id" WHERE "id" = '{}' AND "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_authors_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_authors(page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM "books_authors" INNER JOIN "persons" ON "id" = "books_authors"."person_id" WHERE "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_authors_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_authors(page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM "books_authors" INNER JOIN "persons" ON "id" = "books_authors"."person_id" WHERE "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
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
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM "books_editors" INNER JOIN "persons" ON "books_editors"."person_id" = "persons"."id" WHERE "id" = '{}'"#,
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
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_editors_by_book_id(book_id, page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM "books_editors" INNER JOIN "persons" ON "person_id" = "persons"."id" INNER JOIN "books" ON "book_id" = "books"."id" WHERE "id" = '{}' AND "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_editors_by_book_id(book_id, page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM "books_editors" INNER JOIN "persons" ON "person_id" = "persons"."id" INNER JOIN "books" ON "book_id" = "books"."id" WHERE "id" = '{}' AND "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_editors_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_editors(page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM "books_editors" INNER JOIN "persons" ON "id" = "books_editors"."person_id" WHERE "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_editors_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_editors(page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM (SELECT "id", "first_name", "last_name", "date_of_birth", "isni", "orcid", "oclc" FROM "books_editors" INNER JOIN "persons" ON "id" = "books_editors"."person_id" WHERE "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
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
                r#"SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" WHERE "id" = '{}'"#,
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
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_copies_by_book_id(book_id, page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM (SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM "copies" INNER JOIN "copies" ON "book_id" = "books"."id" WHERE "id" = '{}' AND "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_copies_by_book_id(book_id, page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM (SELECT "id", "book_id", "code_identifier_copy_id", "created_at", "created_by" FROM "copies" INNER JOIN "copies" ON "book_id" = "books"."id" WHERE "id" = '{}' AND "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                book_id, filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_copies_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_copies(page_filter);

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
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_copies(page_filter);

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
                r#"SELECT "id", "code_identifier", "isbn", "issn", "release_year", "edition", "pages", "title", "subtitle", "description" WHERE "id" = '{}'"#,
                id
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_books_page_after() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::After(filter_id), items);
        let query = get_books(page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "code_identifier", "isbn", "issn", "release_year", "edition", "pages", "subtitle", "title", "description" FROM (SELECT "id", "code_identifier", "isbn", "issn", "release_year", "edition", "pages", "subtitle", "title", "description" FROM "books" WHERE "id" > '{}' ORDER BY "id" ASC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }

    #[test]
    fn ut_get_books_page_before() {
        let filter_id = Uuid::new_v4();
        let items = 10;
        let page_filter = PageFilter::new(Page::Before(filter_id), items);
        let query = get_books(page_filter);

        assert_eq!(
            format!(
                r#"SELECT "id", "code_identifier", "isbn", "issn", "release_year", "edition", "pages", "subtitle", "title", "description" FROM (SELECT "id", "code_identifier", "isbn", "issn", "release_year", "edition", "pages", "subtitle", "title", "description" FROM "books" WHERE "id" < '{}' ORDER BY "id" DESC LIMIT {}) AS "t" ORDER BY "id" ASC"#,
                filter_id, items
            ),
            query.to_string(PostgresQueryBuilder)
        );
    }
}
