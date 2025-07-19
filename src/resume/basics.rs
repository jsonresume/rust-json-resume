use serde::{Deserialize, Serialize};

mod location;
mod profile;

use location::Location;
use profile::Profile;

#[derive(Debug, Serialize, Deserialize)]
pub struct Basics {
    pub name: Option<String>,
    pub label: Option<String>,
    pub image: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub url: Option<String>,
    pub summary: Option<String>,
    pub location: Option<Location>,
    pub profiles: Vec<Profile>,
}
