use http::status::StatusCode;
use serde::Serialize;
use std::{collections::HashMap, convert::Infallible};
use tarpc::context;
use warp::Reply;

#[derive(Serialize)]
struct ErrorMessage<'a> {
    code: u32,
    message: &'a str,
}

pub async fn get_oauth_client_identifier() -> Result<impl Reply, Infallible> {
    let mut client = match crate::rpc::identity_client().await {
        Ok(val) => val,
        Err(e) => {
            log::error!("Identity service error: {}", e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 500,
                    message: "INTERNAL_SERVER_ERROR",
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    client
        .oauth_client_identifier(context::current())
        .await
        .map(|x| match x {
            Ok(val) => warp::reply::with_status(warp::reply::json(&val), StatusCode::OK),
            Err(identity_rpc_service::Error::InternalError) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 500,
                    message: "INTERNAL_SERVER_ERROR",
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Err(identity_rpc_service::Error::NotFound) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 404,
                    message: "NOT_FOUND",
                }),
                StatusCode::NOT_FOUND,
            ),
            Err(identity_rpc_service::Error::AlreadyExists) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 400,
                    message: "ALREADY_EXISTS",
                }),
                StatusCode::BAD_REQUEST,
            ),
            Err(identity_rpc_service::Error::InvalidInput) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 400,
                    message: "INVALID_INPUT",
                }),
                StatusCode::BAD_REQUEST,
            ),
            Err(identity_rpc_service::Error::InvalidData) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 400,
                    message: "INVALID_DATA",
                }),
                StatusCode::BAD_REQUEST,
            ),
        })
        .or_else(|e| {
            log::error!("Identity service error: {}", e);
            Ok(warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 500,
                    message: "INTERNAL_SERVER_ERROR",
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        })
}

pub async fn create_oauth_authentication(
    body: HashMap<String, String>,
) -> Result<impl Reply, Infallible> {
    let mut client = match crate::rpc::identity_client().await {
        Ok(val) => val,
        Err(e) => {
            log::error!("Identity service error: {}", e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 500,
                    message: "INTERNAL_SERVER_ERROR",
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    client
        .oauth_authentication(context::current(), body["code"].clone())
        .await
        .map(|x| match x {
            Ok(val) => warp::reply::with_status(warp::reply::json(&val), StatusCode::OK),
            Err(identity_rpc_service::Error::InternalError) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 500,
                    message: "INTERNAL_SERVER_ERROR",
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Err(identity_rpc_service::Error::NotFound) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 404,
                    message: "NOT_FOUND",
                }),
                StatusCode::NOT_FOUND,
            ),
            Err(identity_rpc_service::Error::AlreadyExists) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 400,
                    message: "ALREADY_EXISTS",
                }),
                StatusCode::BAD_REQUEST,
            ),
            Err(identity_rpc_service::Error::InvalidInput) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 400,
                    message: "INVALID_INPUT",
                }),
                StatusCode::BAD_REQUEST,
            ),
            Err(identity_rpc_service::Error::InvalidData) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 400,
                    message: "INVALID_DATA",
                }),
                StatusCode::BAD_REQUEST,
            ),
        })
        .or_else(|e| {
            log::error!("Identity service error: {}", e);
            Ok(warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 500,
                    message: "INTERNAL_SERVER_ERROR",
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        })
}

pub async fn get_session_info(
    session: crate::middleware::session::Session,
) -> Result<impl Reply, Infallible> {
    let mut client = match crate::rpc::identity_client().await {
        Ok(val) => val,
        Err(e) => {
            log::error!("Identity service error: {}", e);
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 500,
                    message: "INTERNAL_SERVER_ERROR",
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    client
        .session_info(context::current(), session.token)
        .await
        .map(|x| match x {
            Ok(val) => warp::reply::with_status(warp::reply::json(&val), StatusCode::OK),
            Err(identity_rpc_service::Error::InternalError) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 500,
                    message: "INTERNAL_SERVER_ERROR",
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Err(identity_rpc_service::Error::NotFound) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 404,
                    message: "NOT_FOUND",
                }),
                StatusCode::NOT_FOUND,
            ),
            Err(identity_rpc_service::Error::AlreadyExists) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 400,
                    message: "ALREADY_EXISTS",
                }),
                StatusCode::BAD_REQUEST,
            ),
            Err(identity_rpc_service::Error::InvalidInput) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 400,
                    message: "INVALID_INPUT",
                }),
                StatusCode::BAD_REQUEST,
            ),
            Err(identity_rpc_service::Error::InvalidData) => warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 400,
                    message: "INVALID_DATA",
                }),
                StatusCode::BAD_REQUEST,
            ),
        })
        .or_else(|e| {
            log::error!("Identity service communication error: {}", e);
            Ok(warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 500,
                    message: "INTERNAL_SERVER_ERROR",
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        })
}
