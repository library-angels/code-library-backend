use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use once_cell::sync::OnceCell;

pub mod models;
pub mod schema;

pub static DB: OnceCell<Pool<ConnectionManager<PgConnection>>> = OnceCell::new();
