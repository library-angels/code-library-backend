use warp::{Filter, filters::BoxedFilter, Reply};
use crate::db::db_connection::{Db, with_db_state};

pub fn root() -> BoxedFilter<(impl Reply,)> {
    // GET - /
    warp::path::end()
        .and(warp::get())
        .and_then(super::endpoints::root::root)
        .boxed()
}

pub fn identity() -> BoxedFilter<(impl Reply,)> {
    warp::path("identity").and(
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
            .and_then(super::endpoints::identity::users_id)
        )

        // GET - /identity/roles
        .or(warp::path("roles")
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .and_then(super::endpoints::identity::roles_index)
        )

        // GET - /identity/roles/<u32>
        .or(warp::path("roles")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .and_then(super::endpoints::identity::roles_id)
        )

        .or(warp::path("oauth").and(
            // GET - /identity/oauth/client_identifier
            warp::path("client_identifier")
                .and(warp::path::end())
                .and(warp::get())
                .and_then(super::endpoints::identity::oauth_client_identifier)

            // GET - /identity/oauth/authorization_code_exchange
            .or(warp::path("authorization_code_exchange")
                .and(warp::path::end())
                .and(warp::get())
                .and(warp::query())
                .and_then(super::endpoints::identity::oauth_authorization_code_exchange)
            ))
        )

        .or(warp::path("jwt").and(
            // GET - /identity/jwt/info
            warp::path("info")
                .and(warp::path::end())
                .and(warp::get())
                .and_then(super::endpoints::identity::jwt_info)

            // POST - /identity/jwt/refresh
            .or(warp::path("refresh")
                .and(warp::path::end())
                .and(warp::post())
                .and(warp::body::json())
                .and_then(super::endpoints::identity::jwt_refresh)
            ))
        )).boxed()
}

pub fn book(db_state: Db) -> BoxedFilter<(impl Reply,)> {
    use super::query_models::book::BookQuery;
    warp::path("book").and(
        // GET - /book/
        warp::path::end()
            .and(warp::get())
            .and(warp::query::<BookQuery>())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_list)
            .boxed()
        
        //POST - /book/
        .or(warp::path::end()
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::create_book)
            .boxed()
            )

        //DELETE - /book/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path::end())
            .and(warp::delete())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::delete_book)
            .boxed()
            )

        // GET - /book/<u32>
        .or(warp::path::param::<u32>()
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve)
            .boxed()
            )

        // GET - /book/<u32>/designations
        .or(warp::path::param::<u32>()
            .and(warp::path("designations"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_designations)
            .boxed()
            )

        // GET - /book/<u32>/tags
        .or(warp::path::param::<u32>()
            .and(warp::path("tags"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_tags)
            .boxed()
            )

        // GET - /book/<u32>/publishers
        .or(warp::path::param::<u32>()
            .and(warp::path("publishers"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_publishers)
            .boxed()
            )

        // GET - /book/<u32>/authors
        .or(warp::path::param::<u32>()
            .and(warp::path("authors"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_authors)
            .boxed()
            )

        // GET - /book/<u32>/editors
        .or(warp::path::param::<u32>()
            .and(warp::path("editors"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_editors)
            .boxed()
            )

        // GET - /book/<u32>/series
        .or(warp::path::param::<u32>()
            .and(warp::path("series"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_series)
            .boxed()
            )

        // GET - /book/<u32>/languages
        .or(warp::path::param::<u32>()
            .and(warp::path("languages"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_languages)
            .boxed()
            )

        // GET - /book/<u32>/physical_sizes
        .or(warp::path::param::<u32>()
            .and(warp::path("physical_sizes"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_physical_sizes)
            .boxed()
            )

        // GET - /book/<u32>/subject_areas
        .or(warp::path::param::<u32>()
            .and(warp::path("subject_areas"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_subject_areas)
            .boxed()
            )

        // GET - /book/<u32>/copies
        .or(warp::path::param::<u32>()
            .and(warp::path("copies"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_copies)
            .boxed()
            )

        // GET - /book/<u32>/status
        .or(warp::path::param::<u32>()
            .and(warp::path("status"))
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::book_retrieve_status)
            .boxed()
            )

        // GET - /book/designations
        .or(warp::path("designations")
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::designations_list)
            .boxed()
            )

        // GET - /book/designations/<u32>
        .or(warp::path("designations")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::designations_retrieve)
            .boxed()
            )

        // GET - /book/tags
        .or(warp::path("tags")
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::tags_list)
            .boxed()
            )

        // GET - /book/tags/<u32>
        .or(warp::path("tags")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::tags_retrieve)
            .boxed()
            )

        // GET - /book/publishers
        .or(warp::path("publishers")
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::publishers_list)
            .boxed()
            )
        
        // GET - /book/publishers/<u32>
        .or(warp::path("publishers")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::publishers_retrieve)
            .boxed()
            )

        // POST - /book/publishers/
        .or(warp::path("publishers")
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::create_publisher)
            .boxed()
            )

        // GET - /book/authors
        .or(warp::path("authors")
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::authors_list)
            .boxed()
            )

        // GET - /book/authors/<u32>
        .or(warp::path("authors")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::authors_retrieve)
            .boxed()
            )

        // GET - /book/editors
        .or(warp::path("editors")
            .and(warp::path::end())
            .and(warp::get())
	        .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::editors_list)
            .boxed()
            )

        // GET - /book/editors/<u32>
        .or(warp::path("editors")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
	        .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::editors_retrieve)
            .boxed()
            )

        // GET - /book/series
        .or(warp::path("series")
            .and(warp::path::end())
            .and(warp::get())
	        .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::series_list)
            .boxed()
            )

        // GET - /book/series/<u32>
        .or(warp::path("series")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
	        .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::series_retrieve)
            .boxed()
            )

        // GET - /book/languages
        .or(warp::path("languages")
            .and(warp::path::end())
            .and(warp::get())
	        .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::languages_list)
            .boxed()
            )

        // GET - /book/languages/<u32>
        .or(warp::path("languages")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
	        .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::languages_retrieve)
            .boxed()
            )

        // GET - /book/physical_sizes
        .or(warp::path("physical_sizes")
            .and(warp::path::end())
            .and(warp::get())
	        .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::physical_sizes_list)
            .boxed()
            )

        // GET - /book/physical_sizes/<u32>
        .or(warp::path("physical_sizes")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
	        .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::physical_sizes_retrieve)
            .boxed()
            )

        // GET - /book/subject_areas
        .or(warp::path("subject_areas")
            .and(warp::path::end())
            .and(warp::get())
	        .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::subject_areas_list)
            .boxed()
            )

        // GET - /book/subject_areas/<u32>
        .or(warp::path("subject_areas")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
	        .and(with_db_state(db_state.clone()))
            .and_then(super::endpoints::book::subject_areas_retrieve)
            .boxed()
            )
    ).boxed()
}

pub fn borrow() -> BoxedFilter<(impl Reply,)> {
    warp::path("borrow").and(
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
            .and_then(super::endpoints::borrow::active_create)
        )

        // GET - /borrow/active/<u32>
        .or(warp::path("active")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .and_then(super::endpoints::borrow::active_retrieve)
        )

        // GET - /borrow/history
        .or(warp::path("history")
            .and(warp::path::end())
            .and(warp::get())
            .and_then(super::endpoints::borrow::history_list)
        )

        // GET - /borrow/history/<u32>
        .or(warp::path("history")
            .and(warp::path::param::<u32>())
            .and(warp::path::end())
            .and(warp::get())
            .and_then(super::endpoints::borrow::history_retrieve)
        )
    ).boxed()
}

pub fn notification() -> BoxedFilter<(impl Reply,)> {
    warp::path("notification")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(super::endpoints::notification::notification)
        .boxed()
}
