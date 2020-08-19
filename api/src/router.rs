use warp::{filters::BoxedFilter, Filter, Reply};

pub fn router() -> BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .and(warp::get())
        .and_then(crate::endpoints::root::root)
        .boxed()
}
