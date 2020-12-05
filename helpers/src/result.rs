use std::error::Error as StdError;

pub type StdResult<T> = Result<T, Box<dyn StdError>>;
