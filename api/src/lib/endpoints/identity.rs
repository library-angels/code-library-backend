use std::{collections::HashMap, net::SocketAddr};

use tarpc::context;
use warp::{reject::Rejection, Reply};

use identity::rpc::get_rpc_client;

use crate::{rejections::InternalServerError, responses::json_object_reply};

pub async fn get_oauth_client_identifier(addr: SocketAddr) -> Result<impl Reply, Rejection> {
    if let Ok(client) = get_rpc_client(addr).await {
        if let Ok(identifier) = client.oauth_client_identifier(context::current()).await {
            return Ok(json_object_reply(&identifier.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn create_oauth_authentication(
    body: HashMap<String, String>,
    addr: SocketAddr,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = get_rpc_client(addr).await {
        if let Ok(authentication) = client
            .oauth_authentication(context::current(), body["code"].clone())
            .await
        {
            return Ok(json_object_reply(&authentication.unwrap()));
        }
    }
    Err(InternalServerError().into())
}

pub async fn get_session_info(
    addr: SocketAddr,
    session: crate::filters::authorization::Session,
) -> Result<impl Reply, Rejection> {
    if let Ok(client) = get_rpc_client(addr).await {
        if let Ok(session) = client.session_info(context::current(), session.token).await {
            return Ok(json_object_reply(&session.unwrap()));
        }
    }
    Err(InternalServerError().into())
}
