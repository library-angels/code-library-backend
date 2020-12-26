use std::sync::Arc;

use tarpc::context;

use super::service::NotificationService;
use crate::config::Configuration;
use crate::db::DbPool;

#[derive(Clone)]
pub struct NotificationServer {
    conf: Arc<Configuration>,
    db_pool: Arc<DbPool>,
}

impl NotificationServer {
    pub fn new(conf: Arc<Configuration>, db_pool: Arc<DbPool>) -> Self {
        Self { conf, db_pool }
    }
}

#[tarpc::server]
impl NotificationService for NotificationServer {
    async fn notification(self, _: context::Context) {
        unimplemented!();
    }
}
