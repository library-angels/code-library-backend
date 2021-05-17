use crate::endpoints::root;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn root() -> BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .and(warp::get())
        .and_then(root::root)
        .boxed()
}
