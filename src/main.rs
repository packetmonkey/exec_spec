use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use itertools::Itertools;
use serde::Deserialize;

#[derive(Parser, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ArgCommand {
    Render,
    Stat,
    ListContacts,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    command: ArgCommand,
    spec_path: PathBuf,
}

#[derive(Deserialize, Debug, Clone)]
struct Term {
    name: String,
    definition: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Contact {
    id: u8,
    name: String,
    email: Option<String>,
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.email {
            Some(email) => write!(f, "[{}](mailto:{})", self.name, email),
            None => write!(f, "{}", self.name),
        }
    }
}

#[derive(Deserialize, Debug)]
struct TechnicalRequirement {
    requirement_id: String,
    author_id: Option<u8>,
    description: String,
    code_url: Option<String>,
    test_url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct KnownGap {
    requirement_id: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Persona {
    id: u8,
    name: String,
    description: String,
}

#[derive(Deserialize, Clone, Debug)]
struct BusinessRequirement {
    id: String,
    name: String,
    date: toml::value::Datetime,
    note: Option<String>,
    owner_id: Option<u8>,
}

impl BusinessRequirement {
    fn markdown_level(&self) -> usize {
        self.id.matches('.').count() + 3
    }
}

#[derive(Deserialize, Debug)]
struct SpecMeta {
    version: u8,
    date: toml::value::Datetime,
    owner_id: u8,
    description: String,
    contacts: toml::Table,
}

#[derive(Debug)]
struct Spec {
    terms: Vec<Term>,
    contacts: Vec<Contact>,
    technical_requirements: Vec<TechnicalRequirement>,
    known_gaps: Vec<KnownGap>,
    personas: Vec<Persona>,
    business_requirements: Vec<BusinessRequirement>,
    meta: SpecMeta,
    sla: String,
}

impl Spec {
    fn add_term(&mut self, term: Term) {
        self.terms.push(term);
    }

    fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }

    fn add_technical_requirement(&mut self, technical_requirement: TechnicalRequirement) {
        self.technical_requirements.push(technical_requirement);
    }

    fn add_known_gap(&mut self, known_gap: KnownGap) {
        self.known_gaps.push(known_gap);
    }

    fn add_persona(&mut self, persona: Persona) {
        self.personas.push(persona);
    }

    fn add_requirement(&mut self, requirement: BusinessRequirement) {
        self.business_requirements.push(requirement);
    }

    fn find_contact(&self, contact_id: u8) -> &Contact {
        self.contacts
            .iter()
            .find(|&contact| contact.id == contact_id)
            .unwrap()
    }

    fn find_technical_requirements_for(
        &self,
        requirement_id: String,
    ) -> Vec<&TechnicalRequirement> {
        self.technical_requirements
            .iter()
            .filter(|requirement| requirement.requirement_id == requirement_id)
            .collect()
    }
}

fn main() {
    let args = Args::parse();

    let meta_path = args.spec_path.join("meta.toml");
    let content = std::fs::read_to_string(meta_path).unwrap();
    let meta = toml::from_str(&content).unwrap();

    let mut spec = Spec {
        terms: vec![],
        contacts: vec![],
        technical_requirements: vec![],
        known_gaps: vec![],
        personas: vec![],
        business_requirements: vec![],
        meta: meta,
        sla: "".to_string(),
    };

    let terms_path = args.spec_path.join("terms");
    match std::fs::read_dir(terms_path) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                let content = std::fs::read_to_string(path).unwrap();
                let term: Term = toml::from_str(&content).unwrap();

                spec.add_term(term);
            }
        }
        Err(_) => println!("No terms directory found"),
    }

    let contacts_path = args.spec_path.join("contacts");
    match std::fs::read_dir(contacts_path) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                let content = std::fs::read_to_string(path).unwrap();
                let contact: Contact = toml::from_str(&content).unwrap();

                spec.add_contact(contact);
            }
        }
        Err(_) => println!("No contacts directory found"),
    }

    let technical_requirements_path = args.spec_path.join("technical_requirements");
    match std::fs::read_dir(technical_requirements_path) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                let content = std::fs::read_to_string(path).unwrap();
                let technical_requirement: TechnicalRequirement = toml::from_str(&content).unwrap();

                spec.add_technical_requirement(technical_requirement);
            }
        }
        Err(_) => println!("No technical_requirements directory found"),
    }

    let known_gaps_path = args.spec_path.join("known_gaps");
    match std::fs::read_dir(known_gaps_path) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                let content = std::fs::read_to_string(path).unwrap();
                let known_gap: KnownGap = toml::from_str(&content).unwrap();

                spec.add_known_gap(known_gap);
            }
        }
        Err(_) => println!("No known_gaps directory found"),
    }

    let personas_path = args.spec_path.join("personas");
    match std::fs::read_dir(personas_path) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                let content = std::fs::read_to_string(path).unwrap();
                let persona: Persona = toml::from_str(&content).unwrap();

                spec.add_persona(persona);
            }
        }
        Err(_) => println!("No personas directory found"),
    }

    let business_requirements_path = args.spec_path.join("business_requirements");
    match std::fs::read_dir(business_requirements_path) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                let content = std::fs::read_to_string(path).unwrap();
                let requirement: BusinessRequirement = toml::from_str(&content).unwrap();

                spec.add_requirement(requirement);
            }
        }
        Err(_) => println!("No business_requirements directory found"),
    }

    let sla_path = args.spec_path.join("sla").join("sla.md");
    spec.sla = std::fs::read_to_string(sla_path).unwrap();

    match args.command {
        ArgCommand::Render => render(&spec),
        ArgCommand::Stat => stat(&spec),
        ArgCommand::ListContacts => list_contacts(&spec),
    }
}

fn list_contacts(spec: &Spec) {
    let mut sorted_contacts = spec.contacts.clone();
    sorted_contacts.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    for contact in sorted_contacts {
        println!(
            "{}: {} ({})",
            contact.id,
            contact.name,
            contact.email.unwrap_or_default()
        );
    }
}

fn stat(spec: &Spec) {
    println!("Spec Stats");
    println!(
        "Business Requirements: {}",
        spec.business_requirements.len()
    );
    println!(
        "Technical Requirements: {}",
        spec.technical_requirements.len()
    );

    let number_of_requirements_with_technical_requirements = spec
        .technical_requirements
        .iter()
        .map(|tr| tr.requirement_id.clone())
        .unique()
        .collect::<Vec<String>>()
        .len();

    let percentage = (number_of_requirements_with_technical_requirements as f32
        / spec.business_requirements.len() as f32)
        * 100.0;
    println!(
        "Percentage Requirements Technically Specified: {}%",
        percentage
    );
}

fn render(spec: &Spec) {
    println!("# Specification");
    println!("Version: {}", spec.meta.version);
    println!("Date: {}", spec.meta.date);
    println!("Owner: {}", spec.find_contact(spec.meta.owner_id));
    println!("");
    println!("## Description");
    println!("{}", spec.meta.description);
    println!("");
    println!("## Known Gaps");
    for known_gap in &spec.known_gaps {
        println!("### {}", known_gap.requirement_id);
        println!("{}", known_gap.description);
        println!("");
    }
    println!("## Personas");
    for persona in &spec.personas {
        println!("### {}", persona.name);
        println!("{}", persona.description);
        println!("");
    }

    println!("## Requirements");
    let mut sorted_requirements = spec.business_requirements.to_owned().clone();
    sorted_requirements.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
    for requirement in sorted_requirements {
        let markdown_level = requirement.markdown_level();

        println!(
            "{} {}. {}",
            "#".repeat(markdown_level),
            requirement.id,
            requirement.name
        );
        if let Some(note) = requirement.note {
            println!("{}", note);
        }

        let technical_requirements = spec.find_technical_requirements_for(requirement.id);
        if technical_requirements.len() > 0 {
            println!("{} Technical Requirements", "#".repeat(markdown_level + 1));
        }

        for technical_requirement in technical_requirements {
            println!("{}", technical_requirement.description);

            if let Some(author_id) = &technical_requirement.author_id {
                println!("Author: {}", spec.find_contact(*author_id));
            }

            if let Some(url) = &technical_requirement.code_url {
                println!("[Code]({})", url);
            }

            if let Some(url) = &technical_requirement.test_url {
                println!("[Tests]({})", url);
            }

            println!("");
        }
    }
    println!("## SLA");
    println!("{}", spec.sla);

    println!("## Glossary");
    let mut sorted_terms = spec.terms.to_owned().clone();
    sorted_terms.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    for term in sorted_terms {
        println!("### {}", term.name);
        println!("{}", term.definition);
        println!("");
    }

    println!("## Contacts");
    // let mut contact_groups: Vec<&String> =
    for contact_group in spec.meta.contacts.keys() {
        println!("### {}", contact_group);

        let contact_ids = spec
            .meta
            .contacts
            .get(contact_group)
            .unwrap()
            .as_array()
            .unwrap();

        for contact_id in contact_ids {
            let contact_id = contact_id.as_integer().unwrap() as u8;
            println!("- {}", spec.find_contact(contact_id as u8));
        }

        println!("");
    }
}
