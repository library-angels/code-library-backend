use std::convert::Infallible;
use std::collections::HashMap;
use std::sync::Arc;
use serde::Serialize;


#[derive(Serialize)]
struct OauthClientIdentifier {
    client_identifier: String,
}

pub async fn users_index(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("users_index"))
}

pub async fn users_id(user_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("users id: {}", user_id))
}

pub async fn roles_index(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("roles index"))
}

pub async fn roles_id(role_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("roles id: {}", role_id))
}

pub async fn oauth_client_identifier(config: Arc<Box<super::super::config::Config>>) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&OauthClientIdentifier{client_identifier: config.oauth_client_identifier.clone()}))
}

pub async fn oauth_authorization_code_exchange(query: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("oauth_authorization_code_exchange - code: {}", query["code"]))
}

pub async fn jwt_info() -> Result<impl warp::Reply, Infallible> {
    Ok(format!("jwt_info"))
}

pub async fn jwt_refresh(body: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("jwt_refresh"))
}
