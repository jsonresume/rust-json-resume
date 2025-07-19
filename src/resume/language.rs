use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub language: Option<String>,
    pub fluency: Option<String>,
}
