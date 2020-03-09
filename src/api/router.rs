use warp::{Filter, filters::BoxedFilter, Reply};

pub fn root() -> BoxedFilter<(impl Reply,)> {
    warp::path::end().map(|| {format!("root")}).boxed()
}

pub fn identity() -> BoxedFilter<(impl Reply,)> {
    warp::path("identity").map(|| {format!("identity")}).boxed()
}

pub fn book() -> BoxedFilter<(impl Reply,)> {
    warp::path("book").map(|| {format!("book")}).boxed()
}

pub fn borrow() -> BoxedFilter<(impl Reply,)> {
    warp::path("borrow").map(|| {format!("borrow")}).boxed()
}

pub fn notification() -> BoxedFilter<(impl Reply,)> {
    warp::path("notification").map(|| {format!("notification")}).boxed()
}
