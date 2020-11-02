use http::StatusCode;
use serde::Serialize;
use warp::reply::{Json, WithStatus};

#[derive(Clone, Serialize)]
struct ErrorMessage<'a> {
    pub code: u32,
    pub msg: &'a str,
}

/// `200` OK
pub fn okay_with_json(json: &impl Serialize) -> WithStatus<Json> {
    warp::reply::with_status(warp::reply::json(json), StatusCode::OK)
}

/// `400` Bad Request
pub fn bad_request(msg: &str) -> WithStatus<Json> {
    warp::reply::with_status(
        warp::reply::json(&ErrorMessage { code: 400, msg }),
        StatusCode::BAD_REQUEST,
    )
}

/// `401` Unauthorized
pub fn unauthorized(msg: &str) -> WithStatus<Json> {
    warp::reply::with_status(
        warp::reply::json(&ErrorMessage { code: 401, msg }),
        StatusCode::UNAUTHORIZED,
    )
}

/// `404` Not Found
pub fn not_found() -> WithStatus<Json> {
    warp::reply::with_status(
        warp::reply::json(&ErrorMessage {
            code: 404,
            msg: "NOT_FOUND",
        }),
        StatusCode::NOT_FOUND,
    )
}

/// `405` Method Not Allowed
pub fn method_not_allowed() -> WithStatus<Json> {
    warp::reply::with_status(
        warp::reply::json(&ErrorMessage {
            code: 405,
            msg: "METHOD_NOT_ALLOWED",
        }),
        StatusCode::METHOD_NOT_ALLOWED,
    )
}

/// `500` Internal Server Error
pub fn internal_server_error() -> WithStatus<Json> {
    warp::reply::with_status(
        warp::reply::json(&ErrorMessage {
            code: 500,
            msg: "INTERNAL_SERVER_ERROR",
        }),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}
