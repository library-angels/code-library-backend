use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Error {
    AlreadyExists,
    InternalError,
    InvalidData,
    InvalidInput,
    NotFound,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: i32,
}
