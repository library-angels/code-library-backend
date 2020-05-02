#[cfg(test)]
mod tests {
    use crate::{
        db::{db_model::*, helpers::book_endpoint::*, tests::test_db_connection::*},
        query_models::{
            book::{BookQuery, NewBookQuery},
            publisher::PublisherRequest,
        },
    };
    #[tokio::test]
    async fn retrieve_books_with_limit() {
        let db_state = test_db();
        let query = BookQuery {
            limit: Some(2),
            offset: None,
            publisher_id: None,
        };
        let results = find_books(query, &db_state).unwrap();

        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn retrieve_books_with_offset() {
        let db_state = test_db();
        let query = BookQuery {
            limit: None,
            offset: Some(1),
            publisher_id: None,
        };
        let result: Vec<Book> = find_books(query, &db_state)
            .unwrap()
            .into_iter()
            .filter(|book| book.id == 1)
            .collect();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_books_with_publisher() {
        let db_state = test_db();
        let query = BookQuery {
            limit: None,
            offset: None,
            publisher_id: Some(2),
        };
        let result: Vec<Book> = find_books(query, &db_state)
            .unwrap()
            .into_iter()
            .filter(|book| book.publisher_id == 2)
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn new_book() {
        let db_state = test_db();
        let query = NewBookQuery {
            title: "Test Title".to_string(),
            description: "Test Description".to_string(),
            release_year: Some(2020),
            publisher_id: 2,
            designation_id: 4,
            language_id: 3,
            physical_size_id: 2,
        };
        let result = add_new_book(query, &db_state);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn retrieve_book() {
        let db_state = test_db();
        let result: Vec<Book> = find_book(1, &db_state)
            .into_iter()
            .filter(|book| book.id == 1)
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_book_designations() {
        let db_state = test_db();
        let result: Vec<Designation> = find_book_designations(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|designation| designation.name == "DH")
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_book_publishers() {
        let db_state = test_db();
        let result: Vec<Publisher> = find_book_publishers(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|publisher| publisher.id == 1)
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_book_authors() {
        let db_state = test_db();
        let result: Vec<Person> = find_book_authors(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|person| person.id == 1)
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_book_languages() {
        let db_state = test_db();
        let result: Vec<Language> = find_book_languages(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|language| language.language_name == "English")
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_book_physical_sizes() {
        let db_state = test_db();
        let result: Vec<PhysicalSize> = find_book_physical_sizes(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|physical_size| physical_size.id == 1)
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_book_subject_areas() {
        let db_state = test_db();
        let result: Vec<SubjectArea> = find_book_subject_areas(3, &db_state)
            .unwrap()
            .into_iter()
            .filter(|subject_area| subject_area.name == "Computer Science")
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_book_copies() {
        let db_state = test_db();
        let result: Vec<Copy> = find_book_copies(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|copies| copies.code_identifier_copy_id == Some(1))
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_book_status() {
        let db_state = test_db();
        let result: Vec<Status> = find_book_status(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|status| status.available)
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_designations() {
        let db_state = test_db();
        let result: Vec<Designation> = find_all_designations(&db_state).unwrap();

        assert_eq!(result.len(), 6);
    }

    #[tokio::test]
    async fn retrieve_designation() {
        let db_state = test_db();
        let result: Vec<Designation> = find_designation_by_id(2, &db_state)
            .unwrap()
            .into_iter()
            .filter(|designation| designation.name == "SE")
            .collect();

        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn retrieve_publishers() {
        let db_state = test_db();
        let result: Vec<Publisher> = find_all_publishers(&db_state).unwrap();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_publisher() {
        let db_state = test_db();
        let result: Vec<Publisher> = find_publisher_by_id(3, &db_state)
            .unwrap()
            .into_iter()
            .filter(|publisher| publisher.name == "FREEMAN and Company")
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn new_publisher() {
        let db_state = test_db();
        let query = PublisherRequest {
            name: "Test Publisher".to_string(),
        };
        let result = add_new_publisher(query, &db_state);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn retrieve_authors() {
        let db_state = test_db();
        let result: Vec<Person> = find_all_authors(&db_state).unwrap();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_author() {
        let db_state = test_db();
        let result: Vec<Person> = find_author_by_id(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|author| author.last_name == Some("Goldhill".to_string()))
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_languages() {
        let db_state = test_db();
        let result: Vec<Language> = find_all_languages(&db_state).unwrap();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_language() {
        let db_state = test_db();
        let result: Vec<Language> = find_language_by_id(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|language| language.language_name == "English")
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_physical_sizes() {
        let db_state = test_db();
        let result: Vec<PhysicalSize> = find_all_physical_sizes(&db_state).unwrap();

        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn retrieve_physical_size() {
        let db_state = test_db();
        let result: Vec<PhysicalSize> = find_physical_size_by_id(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|physical_size| physical_size.name == "normal")
            .collect();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_subject_areas() {
        let db_state = test_db();
        let result: Vec<SubjectArea> = find_all_subject_areas(&db_state).unwrap();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn retrieve_subject_area() {
        let db_state = test_db();
        let result: Vec<SubjectArea> = find_subject_area_by_id(1, &db_state)
            .unwrap()
            .into_iter()
            .filter(|subject_area| subject_area.name == "Health Care")
            .collect();

        assert!(!result.is_empty());
    }
}
