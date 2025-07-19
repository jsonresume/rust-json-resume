use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub highlights: Vec<String>,
    pub keywords: Vec<String>,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub roles: Vec<String>,
    pub url: String,
    pub entity: String,
    #[serde(rename = "type")]
    pub kind: String,
}
