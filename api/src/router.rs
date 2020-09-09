use crate::middleware::rejection;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn router() -> BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .and(warp::get())
        .and_then(crate::endpoints::root::root)
        .boxed()
        .or(warp::path("identity").and(
            warp::path("oauth")
                .and(
                    // GET - /identity/oauth/client_identifier
                    warp::path("client_identifier")
                        .and(warp::path::end())
                        .and(warp::get())
                        .and_then(crate::endpoints::identity::get_oauth_client_identifier)
                        .boxed()
                        // POST - /identity/oauth/authentication
                        .or(warp::path("authentication")
                            .and(warp::path::end())
                            .and(warp::post())
                            .and(warp::body::json())
                            .and_then(crate::endpoints::identity::create_oauth_authentication)
                            .boxed()),
                )
                .boxed(),
        ))
        .recover(rejection::handle_rejection)
        .boxed()
}
