use crate::{endpoints::identity::*, middleware::session::authorization};
use std::net::SocketAddr;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn identity(identity_addr: SocketAddr) -> BoxedFilter<(impl Reply,)> {
    let identity_filter = warp::any().map(move || identity_addr);

    warp::path("identity")
        .and(
            warp::path("oauth")
                .and(
                    // GET - /identity/oauth/client_identifier
                    warp::path("client_identifier")
                        .and(warp::path::end())
                        .and(warp::get())
                        .and(identity_filter)
                        .and_then(get_oauth_client_identifier)
                        // POST - /identity/oauth/authentication
                        .or(warp::path("authentication")
                            .and(warp::path::end())
                            .and(warp::post())
                            .and(warp::body::json())
                            .and(identity_filter)
                            .and_then(create_oauth_authentication)),
                )
                // GET - /identity/session/info
                .or(warp::path!("session" / "info")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and(identity_filter)
                    .and(authorization(identity_addr))
                    .and_then(get_session_info)),
        )
        .boxed()
}
