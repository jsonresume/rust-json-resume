use serde::{Deserialize, Serialize};

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

use award::Award;
use basics::Basics;
use certificate::Certificate;
use education::Education;
use interest::Interest;
use language::Language;
use project::Project;
use publication::Publication;
use reference::Reference;
use skill::Skill;
use volunteer::Volunteer;
use work::Work;

#[derive(Debug, Serialize, Deserialize)]
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
