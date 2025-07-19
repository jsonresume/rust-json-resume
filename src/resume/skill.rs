use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Skill {
    pub name: Option<String>,
    pub level: Option<String>,
    pub keywords: Vec<String>,
}
