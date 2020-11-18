use diesel::result::Error as DBError;
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

impl From<DBError> for Error {
    fn from(e: DBError) -> Self {
        log::debug!("{}", e);
        match e {
            DBError::NotFound => Error::NotFound,
            DBError::QueryBuilderError(_) => Error::InvalidInput,
            _ => Error::InternalError,
        }
    }
}

pub type RpcResult<T> = Result<T, Error>;
