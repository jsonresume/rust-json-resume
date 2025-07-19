use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    pub institution: Option<String>,
    pub url: Option<String>,
    pub area: Option<String>,
    #[serde(rename = "studyType")]
    pub study_type: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub score: Option<String>,
    pub courses: Option<Vec<String>>,
}
