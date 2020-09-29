use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Reply;

#[derive(Debug, Deserialize, Serialize)]
struct RootInformation<'a> {
    description: &'a str,
    version: &'a str,
}

pub async fn root() -> Result<impl Reply, Infallible> {
    let root = RootInformation {
        description: "CODE Library - REST API",
        version: option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>"),
    };
    Ok(warp::reply::json(&root))
}
