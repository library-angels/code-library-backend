use std::convert::Infallible;

pub async fn users_index(_query: String) -> Result<impl warp::Reply, Infallible> {
    Ok("users_index".to_string())
}

pub async fn users_id(user_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("users id: {}", user_id))
}

pub async fn roles_index(_query: String) -> Result<impl warp::Reply, Infallible> {
    Ok("roles index".to_string())
}

pub async fn roles_id(role_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("roles id: {}", role_id))
}

pub async fn oauth_client_identifier() -> Result<impl warp::Reply, Infallible> {
    Ok("oauth_client_identifier".to_string())
}

pub async fn oauth_authorization_code_exchange(
    query: String,
) -> Result<impl warp::Reply, Infallible> {
    Ok(format!(
        "oauth_authorization_code_exchange - code: {}",
        query
    ))
}

pub async fn jwt_info() -> Result<impl warp::Reply, Infallible> {
    Ok("jwt_info".to_string())
}

pub async fn jwt_refresh(_body: String) -> Result<impl warp::Reply, Infallible> {
    Ok("jwt_refresh".to_string())
}
