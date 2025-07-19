use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Award {
    pub title: String,
    pub date: String,
    pub awarder: String,
    pub summary: String,
}
