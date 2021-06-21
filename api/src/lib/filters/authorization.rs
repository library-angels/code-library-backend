use crate::rejections::{InternalServerError, Unauthorized};

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
            let token = header.strip_prefix("Bearer ").ok_or_else(|| {
                reject::custom(Unauthorized("Authorization header invalid".into()))
            })?;

            let client = get_rpc_client(addr).await.map_err(|e| {
                log::error!("Identity service error: {}", e);
                reject::custom(InternalServerError())
            })?;

            let token_content = client
                .session_info(context::current(), token.into())
                .await
                .map_err(|e| {
                    log::error!("Identity service communication error: {}", e);
                    reject::custom(InternalServerError())
                })?
                .map_err(|_e| {
                    reject::custom(Unauthorized("Authorization header invalid".into()))
                })?;

            Ok::<Session, Rejection>(Session {
                token: token.into(),
                sub: token_content.sub.to_string(),
            })
        },
    )
}
