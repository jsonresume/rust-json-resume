use serde::{Deserialize, Serialize};

mod basics;
mod work;
mod volunteer;
mod education;
mod award;
mod certificate;
mod publication;
mod skill;
mod language;
mod interest;
mod reference;
mod project;

use basics::Basics;
use work::Work;
use volunteer::Volunteer;
use education::Education;
use award::Award;
use certificate::Certificate;
use publication::Publication;
use skill::Skill;
use language::Language;
use interest::Interest;
use reference::Reference;
use project::Project;

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