use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use once_cell::sync::OnceCell;

mod db;
mod rpc;

pub use crate::rpc::*;

pub static DB: OnceCell<Pool<ConnectionManager<PgConnection>>> = OnceCell::new();
