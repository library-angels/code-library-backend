pub(crate) mod models;
pub(crate) mod schema;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub(crate) mod mocked_queries;
        pub(crate) use mocked_queries as queries;
    } else {
        pub(crate) mod queries;
    }
}

pub use helpers::db::*;
