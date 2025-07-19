use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Publication {
    pub name: String,
    pub publisher: String,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    pub url: String,
    pub summary: String,
}