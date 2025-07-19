use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub name: Option<String>,
    pub reference: Option<String>,
}
