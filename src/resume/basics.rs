use serde::{Deserialize, Serialize};

mod location;
mod profile;

use location::Location;
use profile::Profile;

#[derive(Debug, Serialize, Deserialize)]
pub struct Basics {
    pub name: String,
    pub label: String,
    pub image: String,
    pub email: String,
    pub phone: String,
    pub url: String,
    pub summary: String,
    pub location: Location,
    pub profiles: Vec<Profile>,
}