pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use once_cell::sync::OnceCell;

pub static DB: OnceCell<Pool<ConnectionManager<PgConnection>>> = OnceCell::new();
