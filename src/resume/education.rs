use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    pub institution: String,
    pub url: String,
    pub area: String,
    #[serde(rename = "studyType")]
    pub study_type: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub score: String,
    pub courses: Vec<String>,
}