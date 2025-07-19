use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: Option<String>,
    pub description: Option<String>,
    pub highlights: Vec<String>,
    pub keywords: Vec<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub roles: Vec<String>,
    pub url: Option<String>,
    pub entity: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
}
