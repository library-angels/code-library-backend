// extern crate api;

// #[cfg(test)]
// mod tests {
//     #[tokio::test]
//     async fn get_books() {
//         let test_db_state = test_db();
//         let filter = book(test_db_state);
//         let response = warp::test::request()
//             .method("GET")
//             .path("/book/")
//             .reply(&filter)
//             .await;

//         assert_eq!(response.status(), StatusCode::OK);
//     }

//     // #[tokio::test]
//     // async fn get_books_with_limit() {
//     //     let test_db_state = test_db();
//     //     let filter = book(test_db_state);
//     //     let response = warp::test::request()
//     //         .path("/book/")
//     //         .method("GET")
//     //         .body("limit=1")
//     //         .filter(&filter)
//     //         .await;

//     //     assert_eq!(response.len(), 1);
//     // }

// }
