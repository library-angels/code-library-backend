use uuid::Uuid;
use warp::{Filter, Rejection};

use helpers::types::{Page, PageFilter};

#[derive(Debug, serde::Deserialize)]
struct PaginationFilter {
    after: Option<Uuid>,
    before: Option<Uuid>,
    items: Option<i64>,
}

pub fn pagination() -> impl Filter<Extract = (PageFilter,), Error = Rejection> + Clone {
    warp::query::<PaginationFilter>().and_then(|pagination_filter: PaginationFilter| async move {
        if pagination_filter.after.is_some() && pagination_filter.before.is_some() {
            return Err(warp::reject::not_found());
        }

        let page = if pagination_filter.before.is_some() {
            Page::Before(pagination_filter.before.unwrap())
        } else {
            Page::After(pagination_filter.after.unwrap_or_else(Uuid::nil))
        };

        let items = pagination_filter.items.unwrap_or(10);

        Ok::<PageFilter, Rejection>(PageFilter::new(page, items))
    })
}
