use crate::business_requirement::BusinessRequirement;
use crate::business_requirement_id::BusinessRequirementId;
use crate::contact::Contact;
use crate::known_gap::KnownGap;
use crate::persona::Persona;
use crate::spec_meta::SpecMeta;
use crate::technical_requirement::TechnicalRequirement;
use crate::term::Term;

#[derive(Debug)]
pub struct Spec {
    pub terms: Vec<Term>,
    pub contacts: Vec<Contact>,
    pub technical_requirements: Vec<TechnicalRequirement>,
    pub known_gaps: Vec<KnownGap>,
    pub personas: Vec<Persona>,
    pub business_requirements: Vec<BusinessRequirement>,
    pub meta: SpecMeta,
    pub sla: String,
}

impl Spec {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Self {
        let path = path.as_ref();

        let meta_path = path.join("meta.toml");
        let content = match std::fs::read_to_string(&meta_path) {
            Ok(content) => content,
            Err(_) => panic!("Failed to read {}", meta_path.display()),
        };

        let meta: SpecMeta = match toml::from_str(&content) {
            Ok(content) => content,
            Err(e) => panic!("Failed to parse {}: {}", meta_path.display(), e),
        };

        let terms_path = path.join("terms");
        let contacts_path = path.join("contacts");
        let personas_path = path.join("personas");
        let known_gaps_path = path.join("known_gaps");
        let technical_requirements_path = path.join("technical_requirements");
        let business_requirements_path = path.join("business_requirements");

        let sla_path = path.join("sla").join("sla.md");
        let sla = match std::fs::read_to_string(&sla_path) {
            Ok(content) => content,
            Err(_) => panic!("Failed to read {}", sla_path.display()),
        };

        Spec {
            terms: load_directory(terms_path),
            contacts: load_directory(contacts_path),
            technical_requirements: load_directory(technical_requirements_path),
            known_gaps: load_directory(known_gaps_path),
            personas: load_directory(personas_path),
            business_requirements: load_directory(business_requirements_path),
            meta,
            sla,
        }
    }

    pub fn find_contact(&self, contact_id: u8) -> &Contact {
        self.contacts
            .iter()
            .find(|&contact| contact.id == contact_id)
            .unwrap()
    }

    pub fn find_persona(&self, persona_id: u8) -> &Persona {
        self.personas
            .iter()
            .find(|persona| persona.id == persona_id)
            .unwrap()
    }

    pub fn find_technical_requirements_for(
        &self,
        requirement_id: &BusinessRequirementId,
    ) -> Vec<&TechnicalRequirement> {
        self.technical_requirements
            .iter()
            .filter(|requirement| &requirement.requirement_id == requirement_id)
            .collect()
    }
}

fn load_directory<P, T>(path: P) -> Vec<T>
where
    P: AsRef<std::path::Path>,
    T: serde::de::DeserializeOwned,
{
    let path = path.as_ref();

    match std::fs::read_dir(path) {
        Ok(entries) => entries
            .map(Result::unwrap)
            .map(|entry| entry.path())
            .map(std::fs::read_to_string)
            .map(Result::unwrap)
            .map(|str| toml::from_str(&str))
            .map(Result::unwrap)
            .collect(),
        Err(_) => panic!("No directory found at {}", path.display()),
    }
}
