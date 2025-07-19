use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub address: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    pub region: Option<String>,
}
