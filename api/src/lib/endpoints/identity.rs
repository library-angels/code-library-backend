use std::{collections::HashMap, convert::Infallible, net::SocketAddr};

use tarpc::context;
use warp::Reply;

use helpers::rpc::Error;
use identity::rpc::get_rpc_client;

use crate::response;

pub async fn get_oauth_client_identifier(addr: SocketAddr) -> Result<impl Reply, Infallible> {
    let client = match get_rpc_client(addr).await {
        Ok(val) => val,
        Err(e) => {
            log::error!("Identity service error: {}", e);
            return Ok(response::internal_server_error());
        }
    };

    client
        .oauth_client_identifier(context::current())
        .await
        .map(|x| match x {
            Ok(val) => response::okay_with_json(&val),
            Err(Error::InternalError) => response::internal_server_error(),
            Err(Error::NotFound) => response::not_found("NOT_FOUND"),
            Err(Error::AlreadyExists) => response::bad_request("ALREADY_EXISTS"),
            Err(Error::InvalidInput) => response::bad_request("INVALID_INPUT"),
            Err(Error::InvalidData) => response::bad_request("INVALID_DATA"),
        })
        .or_else(|e| {
            log::error!("Identity service error: {}", e);
            Ok(response::internal_server_error())
        })
}

pub async fn create_oauth_authentication(
    body: HashMap<String, String>,
    addr: SocketAddr,
) -> Result<impl Reply, Infallible> {
    let client = match get_rpc_client(addr).await {
        Ok(val) => val,
        Err(e) => {
            log::error!("Identity service error: {}", e);
            return Ok(response::internal_server_error());
        }
    };

    client
        .oauth_authentication(context::current(), body["code"].clone())
        .await
        .map(|x| match x {
            Ok(val) => response::okay_with_json(&val),
            Err(Error::InternalError) => response::internal_server_error(),
            Err(Error::NotFound) => response::not_found("NOT_FOUND"),
            Err(Error::AlreadyExists) => response::bad_request("ALREADY_EXISTS"),
            Err(Error::InvalidInput) => response::bad_request("INVALID_INPUT"),
            Err(Error::InvalidData) => response::bad_request("INVALID_DATA"),
        })
        .or_else(|e| {
            log::error!("Identity service error: {}", e);
            Ok(response::internal_server_error())
        })
}

pub async fn get_session_info(
    addr: SocketAddr,
    session: crate::middleware::session::Session,
) -> Result<impl Reply, Infallible> {
    let client = match get_rpc_client(addr).await {
        Ok(val) => val,
        Err(e) => {
            log::error!("Identity service error: {}", e);
            return Ok(response::internal_server_error());
        }
    };

    client
        .session_info(context::current(), session.token)
        .await
        .map(|x| match x {
            Ok(val) => response::okay_with_json(&val),
            Err(Error::InternalError) => response::internal_server_error(),
            Err(Error::NotFound) => response::not_found("NOT_FOUND"),
            Err(Error::AlreadyExists) => response::bad_request("ALREADY_EXISTS"),
            Err(Error::InvalidInput) => response::bad_request("INVALID_INPUT"),
            Err(Error::InvalidData) => response::bad_request("INVALID_DATA"),
        })
        .or_else(|e| {
            log::error!("Identity service communication error: {}", e);
            Ok(response::internal_server_error())
        })
}
