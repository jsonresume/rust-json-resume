use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Award {
    pub title: Option<String>,
    pub date: Option<String>,
    pub awarder: Option<String>,
    pub summary: Option<String>,
}
