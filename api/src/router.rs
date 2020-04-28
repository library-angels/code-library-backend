use crate::db::db_connection::{with_db_state, Db};
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn root() -> BoxedFilter<(impl Reply,)> {
    // GET - /
    warp::path::end()
        .and(warp::get())
        .and_then(super::endpoints::root::root)
        .boxed()
}

pub fn identity() -> BoxedFilter<(impl Reply,)> {
    warp::path("identity")
        .and(
            // GET - /identity/users
            warp::path("users")
                .and(warp::path::end())
                .and(warp::get())
                .and(warp::query())
                .and_then(super::endpoints::identity::users_index)
                // GET - /identity/users/<u32>
                .or(warp::path("users")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and_then(super::endpoints::identity::users_id))
                // GET - /identity/roles
                .or(warp::path("roles")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(warp::query())
                    .and_then(super::endpoints::identity::roles_index))
                // GET - /identity/roles/<u32>
                .or(warp::path("roles")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and_then(super::endpoints::identity::roles_id))
                // GET - /identity/oauth/client_identifier
                .or(warp::path("oauth").and(
                    warp::path("client_identifier")
                        .and(warp::path::end())
                        .and(warp::get())
                        .and_then(super::endpoints::identity::oauth_client_identifier)
                        // GET - /identity/oauth/authorization_code_exchange
                        .or(warp::path("authorization_code_exchange")
                            .and(warp::path::end())
                            .and(warp::get())
                            .and(warp::query())
                            .and_then(
                                super::endpoints::identity::oauth_authorization_code_exchange,
                            )),
                ))
                // GET - /identity/jwt/info
                .or(warp::path("jwt").and(
                    warp::path("info")
                        .and(warp::path::end())
                        .and(warp::get())
                        .and_then(super::endpoints::identity::jwt_info)
                        // POST - /identity/jwt/refresh
                        .or(warp::path("refresh")
                            .and(warp::path::end())
                            .and(warp::post())
                            .and(warp::body::json())
                            .and_then(super::endpoints::identity::jwt_refresh)),
                )),
        )
        .boxed()
}

pub fn book(db_state: Db) -> BoxedFilter<(impl Reply,)> {
    use super::query_models::book::BookQuery;
    warp::path("book")
        .and(
            // GET - /book/
            warp::path::end()
                .and(warp::get())
                .and(warp::query::<BookQuery>())
                .and(with_db_state(db_state.clone()))
                .and_then(super::endpoints::book::book_list)
                //POST - /book/
                .or(warp::path::end()
                    .and(warp::post())
                    .and(warp::body::json())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::create_book))
                //DELETE - /book/<u32>
                .or(warp::path::param::<u32>()
                    .and(warp::path::end())
                    .and(warp::delete())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::delete_book))
                // GET - /book/<u32>
                .or(warp::path::param::<u32>()
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve))
                // GET - /book/<u32>/designations
                .or(warp::path::param::<u32>()
                    .and(warp::path("designations"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_designations))
                // GET - /book/<u32>/tags
                .or(warp::path::param::<u32>()
                    .and(warp::path("tags"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_tags))
                // GET - /book/<u32>/publishers
                .or(warp::path::param::<u32>()
                    .and(warp::path("publishers"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_publishers))
                // GET - /book/<u32>/authors
                .or(warp::path::param::<u32>()
                    .and(warp::path("authors"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_authors))
                // GET - /book/<u32>/editors
                .or(warp::path::param::<u32>()
                    .and(warp::path("editors"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_editors))
                // GET - /book/<u32>/series
                .or(warp::path::param::<u32>()
                    .and(warp::path("series"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_series))
                // GET - /book/<u32>/languages
                .or(warp::path::param::<u32>()
                    .and(warp::path("languages"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_languages))
                // GET - /book/<u32>/physical_sizes
                .or(warp::path::param::<u32>()
                    .and(warp::path("physical_sizes"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_physical_sizes))
                // GET - /book/<u32>/subject_areas
                .or(warp::path::param::<u32>()
                    .and(warp::path("subject_areas"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_subject_areas))
                // GET - /book/<u32>/copies
                .or(warp::path::param::<u32>()
                    .and(warp::path("copies"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_copies))
                // GET - /book/<u32>/status
                .or(warp::path::param::<u32>()
                    .and(warp::path("status"))
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::book_retrieve_status))
                // GET - /book/designations
                .or(warp::path("designations")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::designations_list))
                // GET - /book/designations/<u32>
                .or(warp::path("designations")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::designations_retrieve))
                // GET - /book/tags
                .or(warp::path("tags")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::tags_list))
                // GET - /book/tags/<u32>
                .or(warp::path("tags")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::tags_retrieve))
                // GET - /book/publishers
                .or(warp::path("publishers")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::publishers_list))
                // GET - /book/publishers/<u32>
                .or(warp::path("publishers")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::publishers_retrieve))
                // POST - /book/publishers/
                .or(warp::path("publishers")
                    .and(warp::path::end())
                    .and(warp::post())
                    .and(warp::body::json())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::create_publisher))
                // GET - /book/authors
                .or(warp::path("authors")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::authors_list))
                // GET - /book/authors/<u32>
                .or(warp::path("authors")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::authors_retrieve))
                // GET - /book/editors
                .or(warp::path("editors")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::editors_list))
                // GET - /book/editors/<u32>
                .or(warp::path("editors")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::editors_retrieve))
                // GET - /book/series
                .or(warp::path("series")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::series_list))
                // GET - /book/series/<u32>
                .or(warp::path("series")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::series_retrieve))
                // GET - /book/languages
                .or(warp::path("languages")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::languages_list))
                // GET - /book/languages/<u32>
                .or(warp::path("languages")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::languages_retrieve))
                // GET - /book/physical_sizes
                .or(warp::path("physical_sizes")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::physical_sizes_list))
                // GET - /book/physical_sizes/<u32>
                .or(warp::path("physical_sizes")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::physical_sizes_retrieve))
                // GET - /book/subject_areas
                .or(warp::path("subject_areas")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state.clone()))
                    .and_then(super::endpoints::book::subject_areas_list))
                // GET - /book/subject_areas/<u32>
                .or(warp::path("subject_areas")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(with_db_state(db_state))
                    .and_then(super::endpoints::book::subject_areas_retrieve)),
        )
        .boxed()
}

pub fn borrow() -> BoxedFilter<(impl Reply,)> {
    warp::path("borrow")
        .and(
            // GET - /borrow/active
            warp::path("active")
                .and(warp::path::end())
                .and(warp::get())
                .and_then(super::endpoints::borrow::active_list)
                // POST /borrow/active
                .or(warp::path("active")
                    .and(warp::path::end())
                    .and(warp::post())
                    .and(warp::body::json())
                    .and_then(super::endpoints::borrow::active_create))
                // GET - /borrow/active/<u32>
                .or(warp::path("active")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and_then(super::endpoints::borrow::active_retrieve))
                // GET - /borrow/history
                .or(warp::path("history")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and_then(super::endpoints::borrow::history_list))
                // GET - /borrow/history/<u32>
                .or(warp::path("history")
                    .and(warp::path::param::<u32>())
                    .and(warp::path::end())
                    .and(warp::get())
                    .and_then(super::endpoints::borrow::history_retrieve)),
        )
        .boxed()
}

pub fn notification() -> BoxedFilter<(impl Reply,)> {
    warp::path("notification")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(super::endpoints::notification::notification)
        .boxed()
}
