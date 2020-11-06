use serde::{Deserialize, Serialize};

/// Error type returned to API
pub use helpers::rpc::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: i32,
}
