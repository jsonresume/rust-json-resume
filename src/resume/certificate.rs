use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    pub name: String,
    pub date: String,
    pub issuer: String,
    pub url: String,
}
