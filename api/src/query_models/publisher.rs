use crate::db::schema::publishers;
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "publishers"]
pub struct PublisherRequest {
    pub name: String,
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize, Clone)]
#[table_name = "publishers"]
pub struct NewPublisher {
    pub name: String,
}

impl From<PublisherRequest> for NewPublisher {
    fn from(query: PublisherRequest) -> Self {
        Self { name: query.name }
    }
}
