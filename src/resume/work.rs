use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    pub name: String,
    pub description: String,
    pub position: String,
    pub url: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub summary: String,
    pub highlights: Vec<String>,
}