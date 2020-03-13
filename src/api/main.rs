use warp::Filter;

mod router;


#[tokio::main]
async fn main() {
    let routes = router::root()
        .or(router::identity())
        .or(router::book())
        .or(router::borrow())
        .or(router::notification());

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
