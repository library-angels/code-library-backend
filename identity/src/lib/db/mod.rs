pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type Db = Pool<ConnectionManager<PgConnection>>;
