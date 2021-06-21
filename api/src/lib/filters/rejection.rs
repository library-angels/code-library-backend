use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{
    reply::{json, with_status},
    Rejection, Reply,
};

use crate::rejections::*;

#[derive(Debug, Serialize)]
struct ErrorReply<'a> {
    error: &'a Error<'a>,
}

fn error_reply(error: &Error) -> impl Reply {
    json(&ErrorReply { error })
}

#[derive(Debug, Serialize)]
struct Error<'a> {
    code: u32,
    status: &'a str,
    detail: &'a str,
}

pub async fn rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let reply = if err.is_not_found() {
        with_status(
            error_reply(&Error {
                code: 404,
                status: "NOT_FOUND",
                detail: "",
            }),
            StatusCode::NOT_FOUND,
        )
    } else if let Some(error) = err.find::<BadRequest>() {
        with_status(
            error_reply(&Error {
                code: 400,
                status: "BAD_REQUEST",
                detail: error.detail(),
            }),
            StatusCode::BAD_REQUEST,
        )
    } else if let Some(error) = err.find::<Unauthorized>() {
        with_status(
            error_reply(&Error {
                code: 401,
                status: "UNAUTHORIZED",
                detail: error.detail(),
            }),
            StatusCode::UNAUTHORIZED,
        )
    } else if err.find::<InternalServerError>().is_some() {
        with_status(
            error_reply(&Error {
                code: 500,
                status: "INTERNAL_SERVER_ERROR",
                detail: "",
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    } else if err.find::<BodyDeserializeError>().is_some() {
        with_status(
            error_reply(&Error {
                code: 400,
                status: "BAD_REQUEST",
                detail: "Invalid body",
            }),
            StatusCode::BAD_REQUEST,
        )
    } else if let Some(error) = err.find::<InvalidHeader>() {
        with_status(
            error_reply(&Error {
                code: 400,
                status: "BAD_REQUEST",
                detail: &format!("Invalid header: {}", error.name()),
            }),
            StatusCode::BAD_REQUEST,
        )
    } else if err.find::<InvalidQuery>().is_some() {
        with_status(
            error_reply(&Error {
                code: 400,
                status: "BAD_REQUEST",
                detail: "Invalid query parameters",
            }),
            StatusCode::BAD_REQUEST,
        )
    } else if err.find::<MethodNotAllowed>().is_some() {
        with_status(
            error_reply(&Error {
                code: 400,
                status: "BAD_REQUEST",
                detail: "Method not allowed",
            }),
            StatusCode::BAD_REQUEST,
        )
    } else if let Some(error) = err.find::<MissingCookie>() {
        with_status(
            error_reply(&Error {
                code: 400,
                status: "BAD_REQUEST",
                detail: &format!("Invalid header: {}", error.name()),
            }),
            StatusCode::BAD_REQUEST,
        )
    } else if let Some(error) = err.find::<MissingHeader>() {
        with_status(
            error_reply(&Error {
                code: 400,
                status: "BAD_REQUEST",
                detail: &format!("Invalid header: {}", error.name()),
            }),
            StatusCode::BAD_REQUEST,
        )
    } else if err.find::<UnsupportedMediaType>().is_some() {
        with_status(
            error_reply(&Error {
                code: 400,
                status: "BAD_REQUEST",
                detail: "Unsupported media type",
            }),
            StatusCode::BAD_REQUEST,
        )
    } else {
        with_status(
            error_reply(&Error {
                code: 500,
                status: "INTERNAL_SERVER_ERROR",
                detail: "",
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    };

    Ok(reply)
}
