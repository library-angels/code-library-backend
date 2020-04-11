use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, Builder};
use diesel::pg::PgConnection;
use warp::Filter;
use log::info;
use std::env;

pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type PgConnectionBuilder = Builder<ConnectionManager<PgConnection>>;
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {
    pub fn new(db_url: &str) -> Self {
        Self::init_pool(db_url, Pool::builder())
    }
    pub fn get_connection(&self) -> PgPooledConnection {
        self.pool.get().unwrap()
    }
    fn init_pool(db_url: &str, builder: PgConnectionBuilder) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = builder.build(manager).expect("can't create db pool");
        Db{ pool }
    }
}

pub async fn start_db(db_url: String) -> Db {
    info!("Starting db from: {}", db_url);
    self::Db::new(&db_url)
}

pub fn with_db_state(
    db_state: Db,
) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_state.clone())
}
