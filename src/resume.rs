use serde::{Deserialize, Serialize};
use serde_valid::Validate;

mod award;
mod basics;
mod certificate;
mod education;
mod interest;
mod language;
mod project;
mod publication;
mod reference;
mod skill;
mod volunteer;
mod work;

pub use award::Award;
pub use basics::Basics;
pub use basics::Location;
pub use basics::Profile;
pub use certificate::Certificate;
pub use education::Education;
pub use interest::Interest;
pub use language::Language;
pub use project::Project;
pub use publication::Publication;
pub use reference::Reference;
pub use skill::Skill;
pub use volunteer::Volunteer;
pub use work::Work;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Resume {
    pub basics: Basics,
    pub work: Vec<Work>,
    pub volunteer: Vec<Volunteer>,
    pub education: Vec<Education>,
    pub awards: Vec<Award>,
    pub certificates: Vec<Certificate>,
    pub publications: Vec<Publication>,
    pub skills: Vec<Skill>,
    pub languages: Vec<Language>,
    pub interests: Vec<Interest>,
    pub references: Vec<Reference>,
    pub projects: Vec<Project>,
}
