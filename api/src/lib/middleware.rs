pub mod session {
    use std::net::SocketAddr;

    use tarpc::context;
    use warp::{reject, Filter, Rejection};

    use identity::rpc::get_rpc_client;

    pub struct Session {
        pub token: String,
        pub sub: String,
    }

    pub fn authorization(
        addr: SocketAddr,
    ) -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
        let addr = warp::any().map(move || addr);
        warp::header::<String>("authorization").and(addr).and_then(
            |header: String, addr: SocketAddr| async move {
                let token = header
                    .strip_prefix("Bearer ")
                    .ok_or_else(|| reject::custom(super::rejection::NotAuthenticated))?;

                let mut client = get_rpc_client(addr).await.map_err(|e| {
                    log::error!("Identity service error: {}", e);
                    reject::custom(super::rejection::NotAuthenticated)
                })?;

                let token_content = client
                    .session_info(context::current(), token.into())
                    .await
                    .map_err(|e| {
                        log::error!("Identity service communication error: {}", e);
                        reject::custom(super::rejection::NotAuthenticated)
                    })?
                    .map_err(|_e| reject::custom(super::rejection::NotAuthenticated))?;

                Ok::<Session, Rejection>(Session {
                    token: token.into(),
                    sub: token_content.sub.to_string(),
                })
            },
        )
    }
}

pub mod rejection {
    use std::convert::Infallible;
    use warp::filters::body::BodyDeserializeError;
    use warp::{reject, reject::MethodNotAllowed, Rejection, Reply};

    use crate::response;

    #[derive(Debug)]
    pub struct NotAuthenticated;

    impl reject::Reject for NotAuthenticated {}

    pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
        let reply = if err.is_not_found() {
            response::not_found("NOT_FOUND")
        } else if let Some(_not_authenticated) = err.find::<NotAuthenticated>() {
            response::unauthorized("AUTHENTICATION_REQUIRED")
        } else if err.find::<BodyDeserializeError>().is_some() {
            response::bad_request("BAD_REQUEST")
        } else if err.find::<MethodNotAllowed>().is_some() {
            response::method_not_allowed()
        } else {
            response::internal_server_error()
        };

        Ok(reply)
    }
}
