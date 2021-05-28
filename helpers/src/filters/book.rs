use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    categories: Option<Vec<String>>,
    publishers: Option<Vec<String>>,
    series: Option<Vec<String>>,
    tags: Option<Vec<String>>,
}

impl Book {
    pub fn new(
        categories: Option<Vec<String>>,
        publishers: Option<Vec<String>>,
        series: Option<Vec<String>>,
        tags: Option<Vec<String>>,
    ) -> Book {
        Self {
            categories,
            publishers,
            series,
            tags,
        }
    }

    pub fn get_categories(&self) -> Option<Vec<String>> {
        self.categories.clone()
    }

    pub fn get_publishers(&self) -> Option<Vec<String>> {
        self.publishers.clone()
    }

    pub fn get_series(&self) -> Option<Vec<String>> {
        self.series.clone()
    }

    pub fn get_tags(&self) -> Option<Vec<String>> {
        self.tags.clone()
    }
}
