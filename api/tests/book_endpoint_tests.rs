extern crate api;

#[cfg(test)]
mod tests {
    use api::{
        db::tests::test_db_connection::*,
        query_models::{book::*, publisher::*},
        router::book,
    };
    use warp::{http::StatusCode, test::request};

    #[tokio::test]
    async fn get() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request().method("GET").path("/book/").reply(&filter).await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_limit() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/")
            .json(&BookQuery {
                limit: Some(1),
                offset: None,
                publisher_id: None,
                designation_id: None,
            })
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_offset() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/")
            .json(&BookQuery {
                limit: None,
                offset: Some(1),
                publisher_id: None,
                designation_id: None,
            })
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_publisher_id() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/")
            .json(&BookQuery {
                limit: None,
                offset: None,
                publisher_id: Some(2),
                designation_id: None,
            })
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn post() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("POST")
            .path("/book/")
            .json(&NewBookQuery {
                title: "Test Title".to_string(),
                description: "Test Description".to_string(),
                release_year: Some(2020),
                publisher_id: 2,
                designation_id: 4,
                language_id: 3,
                physical_size_id: 2,
            })
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request().method("GET").path("/book/1").reply(&filter).await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_designations() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/designations")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_tags() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/tags")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_publishers() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/publishers")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_authors() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/authors")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_editors() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/editors")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_series() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/series")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_languages() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/languages")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_physical_sizes() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/physical_sizes")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_subject_areas() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/subject_areas")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_copies() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/copies")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_with_id_status() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/1/status")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_designations() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/designations")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_designation() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/designations/1")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_tags() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/tags")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_tag() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/tags/1")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_publishers() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/publishers")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_publisher() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/publishers/1")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn post_publisher() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("POST")
            .json(&NewPublisher {
                name: "Test Publisher".to_string(),
            })
            .path("/book/publishers")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_authors() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/authors")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_author() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/authors/1")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_series() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/series")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_series_with_id() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/series/1")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_languages() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/languages")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_language() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/languages/1")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_physical_sizes() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/physical_sizes")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_physical_size() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/physical_sizes/1")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_subject_areas() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/subject_areas")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn get_subject_area() {
        let test_db_state = test_db();
        let filter = book(test_db_state);
        let response = request()
            .method("GET")
            .path("/book/subject_areas/1")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }
}
