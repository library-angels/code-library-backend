use serde::{Deserialize, Serialize};

pub use helpers::rpc::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: i32,
}
