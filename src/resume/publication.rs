use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Publication {
    pub name: Option<String>,
    pub publisher: Option<String>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    pub url: Option<String>,
    pub summary: Option<String>,
}
