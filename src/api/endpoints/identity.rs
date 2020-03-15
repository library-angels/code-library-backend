use std::convert::Infallible;
use std::collections::HashMap;


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

pub async fn oauth_client_identifier() -> Result<impl warp::Reply, Infallible> {
    Ok(format!("oauth_client_identifier"))
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
