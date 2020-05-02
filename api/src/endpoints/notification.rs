use std::convert::Infallible;

pub async fn notification() -> Result<impl warp::Reply, Infallible> {
    Ok("notification".to_string())
}
