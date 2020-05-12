use std::convert::Infallible;

pub async fn root() -> Result<impl warp::Reply, Infallible> {
    Ok("root".to_string())
}
