use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    pub name: Option<String>,
    pub date: Option<String>,
    pub issuer: Option<String>,
    pub url: Option<String>,
}
