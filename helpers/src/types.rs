use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Page {
    After(Uuid),
    Before(Uuid),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageFilter {
    page: Page,
    items: i64,
}

impl PageFilter {
    pub fn new(page: Page, items: i64) -> Self {
        Self { page, items }
    }

    pub fn get_page(&self) -> Page {
        self.page.clone()
    }

    pub fn get_items(&self) -> i64 {
        self.items
    }
}
