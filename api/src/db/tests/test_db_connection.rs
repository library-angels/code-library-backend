use crate::{config::initialize_config, db::db_connection::Db};
use diesel::r2d2::{CustomizeConnection, Pool};
use log::error;

#[allow(dead_code)]
pub fn test_db() -> Db {
    let config = initialize_config().unwrap();
    let db_url = config.database_url.clone();
    let customizer = TestConnectionCustomizer {};
    let builder = Pool::builder().connection_customizer(Box::new(customizer));
    Db::init_pool(&db_url, builder)
}

#[derive(Debug)]
pub struct TestConnectionCustomizer;

impl<C, E> CustomizeConnection<C, E> for TestConnectionCustomizer
where
    C: diesel::connection::Connection,
    E: std::error::Error + Sync + Send,
{
    fn on_acquire(&self, connection: &mut C) -> Result<(), E> {
        if let Err(e) = connection.begin_test_transaction() {
            error!("Can't start test transaction: {}", e);
        }
        Ok(())
    }
}
