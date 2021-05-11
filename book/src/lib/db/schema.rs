use sea_query::Iden;

#[derive(Iden)]
pub(crate) enum Categories {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub(crate) enum Languages {
    Table,
    Id,
    IsoCode,
    Name,
}

#[derive(Iden)]
pub(crate) enum Persons {
    Table,
    Id,
    FirstName,
    LastName,
    DateOfBirth,
    Isni,
    Orcid,
    Oclc,
}

#[derive(Iden)]
pub(crate) enum Publishers {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub(crate) enum Series {
    Table,
    Id,
    PublisherId,
    Name,
}

#[derive(Iden)]
pub(crate) enum SubjectAreas {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub(crate) enum Tags {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub(crate) enum Books {
    Table,
    Id,
    CodeIdentifier,
    Isbn,
    Issn,
    ReleaseYear,
    Edition,
    Pages,
    Subtitle,
    Title,
    Description,
    CategoryId,
    LanguageId,
    PublisherId,
    SeriesId,
}

#[derive(Iden)]
#[allow(dead_code)]
pub(crate) enum BooksAuthors {
    Table,
    Id,
    BookId,
    PersonId,
}

#[derive(Iden)]
#[allow(dead_code)]
pub(crate) enum BooksEditors {
    Table,
    Id,
    BookId,
    PersonId,
}

#[derive(Iden)]
#[allow(dead_code)]
pub(crate) enum BooksSubjectAreas {
    Table,
    Id,
    BookId,
    SubjectAreaId,
}

#[derive(Iden)]
#[allow(dead_code)]
pub(crate) enum BooksTags {
    Table,
    Id,
    BookId,
    TagId,
}

#[derive(Iden)]
pub(crate) enum Copies {
    Table,
    Id,
    BookId,
    CodeIdentifierCopyId,
    CreatedAt,
    CreatedBy,
}
