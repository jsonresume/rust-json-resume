use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    pub name: Option<String>,
    pub description: Option<String>,
    pub position: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub summary: Option<String>,
    pub highlights: Vec<String>,
}
