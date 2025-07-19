use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Interest {
    pub name: Option<String>,
    pub keywords: Vec<String>,
}
