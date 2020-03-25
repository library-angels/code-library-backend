use warp::Filter;

mod router;
mod endpoints;


#[tokio::main]
async fn main() {
    env_logger::init();

    let routes = router::root()
        .or(router::identity())
        .or(router::book())
        .or(router::borrow())
        .or(router::notification());

    warp::serve(routes)
        .try_bind(([127, 0, 0, 1], 8080))
        .await;
}
