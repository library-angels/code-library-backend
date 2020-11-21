use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use once_cell::sync::OnceCell;

#[macro_use]
extern crate diesel;

mod authentication;
mod config;
mod db;
mod rpc;
mod session;

pub use crate::config::Configuration;
pub use crate::rpc::*;

pub static CONFIGURATION: OnceCell<Configuration> = OnceCell::new();
pub static DB: OnceCell<Pool<ConnectionManager<PgConnection>>> = OnceCell::new();
