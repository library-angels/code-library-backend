use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
/// Error type to be used in-between api and rpc-services
pub enum Error {
    AlreadyExists,
    InternalError,
    InvalidData,
    InvalidInput,
    NotFound,
}

pub type RpcResult<T> = Result<T, Error>;
