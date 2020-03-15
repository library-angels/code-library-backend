use std::convert::Infallible;
use std::collections::HashMap;


pub async fn active_list() -> Result<impl warp::Reply, Infallible> {
    Ok(format!("active index"))
}

pub async fn active_create(body: HashMap<String, String>) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("active index post"))
}

pub async fn active_retrieve(borrow_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("active id: {}", borrow_id))
}

pub async fn history_list() -> Result<impl warp::Reply, Infallible> {
    Ok(format!("history index"))
}

pub async fn history_retrieve(borrow_id: u32) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("history id: {}", borrow_id))
}