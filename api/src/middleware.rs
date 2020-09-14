pub mod session {
    use tarpc::context;
    use warp::{reject, Filter, Rejection};

    pub struct Session {
        pub token: String,
        pub sub: String,
    }

    pub fn authorization() -> impl Filter<Extract = (Session,), Error = Rejection> + Copy {
        warp::header::<String>("authorization").and_then(|header: String| async move {
            let token = header
                .strip_prefix("Bearer ")
                .ok_or_else(|| reject::custom(super::rejection::NotAuthenticated))?;

            let mut client = crate::rpc::client_connections::identity_client()
                .await
                .map_err(|e| {
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
        })
    }
}

pub mod rejection {
    use http::status::StatusCode;
    use serde::Serialize;
    use std::convert::Infallible;
    use warp::{reject, Rejection, Reply};

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
        } else if err.find::<warp::filters::body::BodyDeserializeError>().is_some() {
            code = StatusCode::BAD_REQUEST;
            message = "BAD_REQUEST";
        } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
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
