use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub network: Option<String>,
    pub username: Option<String>,
    pub url: Option<String>,
}
