use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub address: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    pub city: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub region: String,
}
