use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Cursor {
    #[serde(rename = "after")]
    After(Uuid),
    #[serde(rename = "before")]
    Before(Uuid),
}

impl Default for Cursor {
    fn default() -> Self {
        Self::After(Uuid::nil())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Items(i64);

impl Default for Items {
    fn default() -> Self {
        Items(10)
    }
}

impl Items {
    pub fn new(items: i64) -> Self {
        Self(items)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    #[serde(default)]
    cursor: Cursor,
    #[serde(default)]
    items: Items,
}

impl Page {
    pub fn new(cursor: Cursor, items: Items) -> Self {
        Self { cursor, items }
    }

    pub fn get_cursor(&self) -> Cursor {
        self.cursor.clone()
    }

    pub fn get_items(&self) -> i64 {
        self.items.0
    }
}
