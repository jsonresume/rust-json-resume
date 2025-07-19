use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: String,
    pub keywords: Vec<String>,
}
