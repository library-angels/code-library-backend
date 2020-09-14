pub mod session {
    use warp::{Filter, reject, Rejection};
    use tarpc::context;

    pub struct Session {
        pub token: String,
        pub sub: String,
    }

    pub fn authorization() -> impl Filter<Extract = (Session,), Error = Rejection> + Copy {
        warp::header::<String>("authorization")
            .and_then(|header: String| async move {
                let token = header.strip_prefix("Bearer ").ok_or(reject::custom(super::rejection::NotAuthenticated))?;

                let mut client = crate::rpc::client_connections::identity_client().await.map_err(|e| {
                    log::error!("Identity service error: {}", e);
                    reject::custom(super::rejection::NotAuthenticated)
                })?;

                let token_content = client.session_info(context::current(), token.into()).await.map_err(|e| {
                    log::error!("Identity service communication error: {}", e);
                    reject::custom(super::rejection::NotAuthenticated)
                })?
                .map_err(|_e| reject::custom(super::rejection::NotAuthenticated))?;

                Ok::<Session, Rejection>(Session {token: token.into(), sub: token_content.sub.to_string()})
            })
    }
}

pub mod rejection {
    use warp::{reject, Rejection, Reply};
    use serde::Serialize;
    use http::status::StatusCode;
    use std::convert::Infallible;

    #[derive(Debug)]
    pub struct NotAuthenticated;

    impl reject::Reject for NotAuthenticated {}

    #[derive(Serialize)]
    struct ErrorMessage {
        code: u16,
        message: String,
    }

    pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
        let code;
        let message;

        if err.is_not_found() {
            code = StatusCode::NOT_FOUND;
            message = "NOT_FOUND";
        } else if let Some(_not_authenticated) = err.find::<NotAuthenticated>() {
            code = StatusCode::UNAUTHORIZED;
            message = "AUTHENTICATION_REQUIRED";
        } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
            code = StatusCode::BAD_REQUEST;
            message = "BAD_REQUEST";
        } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
            code = StatusCode::METHOD_NOT_ALLOWED;
            message = "METHOD_NOT_ALLOWED";
        } else {
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "UNHANDLED_REJECTION";
        }

        let json = warp::reply::json(&ErrorMessage {
            code: code.as_u16(),
            message: message.into(),
        });

        Ok(warp::reply::with_status(json, code))
    }
}
