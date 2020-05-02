use std::convert::Infallible;

pub async fn active_list() -> Result<impl warp::Reply, Infallible> {
    Ok("active index".to_string())
}

pub async fn active_create(_body: String) -> Result<impl warp::Reply, Infallible> {
    Ok("active index post".to_string())
}

pub async fn active_retrieve(borrow_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("active id: {}", borrow_id))
}

pub async fn history_list() -> Result<impl warp::Reply, Infallible> {
    Ok("history index".to_string())
}

pub async fn history_retrieve(borrow_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("history id: {}", borrow_id))
}
