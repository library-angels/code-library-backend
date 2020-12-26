use std::sync::Arc;

use tarpc::context;

use super::service::BorrowService;
use crate::config::Configuration;
use crate::db::DbPool;

#[derive(Clone)]
pub struct BorrowServer {
    conf: Arc<Configuration>,
    db_pool: Arc<DbPool>,
}

impl BorrowServer {
    pub fn new(conf: Arc<Configuration>, db_pool: Arc<DbPool>) -> Self {
        Self { conf, db_pool }
    }
}

#[tarpc::server]
impl BorrowService for BorrowServer {
    async fn borrow(self, _: context::Context) {
        unimplemented!();
    }
}
