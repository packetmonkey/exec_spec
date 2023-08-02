use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use itertools::Itertools;

mod business_requirement;
mod business_requirement_id;
mod contact;
mod known_gap;
mod persona;
mod spec;
mod spec_meta;
mod technical_requirement;
mod term;

use business_requirement_id::BusinessRequirementId;
use spec::Spec;

#[derive(Parser, Clone, ValueEnum)]
enum ArgCommand {
    Render,
    Stat,
    ListContacts,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    command: ArgCommand,
    spec_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let spec = Spec::new(args.spec_path);

    match args.command {
        ArgCommand::Render => render(&spec),
        ArgCommand::Stat => stat(&spec),
        ArgCommand::ListContacts => list_contacts(&spec),
    }
}

// Foo bar baz This is just a test to see if the fancy comments, work and They do.
fn list_contacts(spec: &Spec) {
    let name_length = spec
        .contacts
        .iter()
        .map(|c| c.name.len())
        .max()
        .unwrap_or_default();

    for contact in spec.contacts.iter().sorted_by_key(|c| c.id) {
        println!(
            "{:<3} {:width$} {}",
            contact.id,
            contact.name,
            contact.email.clone().unwrap_or("".to_string()),
            width = name_length + 1
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
        .collect::<Vec<BusinessRequirementId>>()
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
    println!("Version: {}\n", spec.meta.version);
    println!("Date: {}\n", spec.meta.date);
    println!("Owner: {}\n", spec.find_contact(spec.meta.owner_id));
    println!();
    println!("## Description");
    println!("{}", spec.meta.description);
    println!();

    if !spec.known_gaps.is_empty() {
        println!("## Known Gaps");
        for known_gap in spec.known_gaps.iter().sorted_by_key(|k| &k.requirement_id) {
            println!("### {}", known_gap.requirement_id);
            println!("{}", known_gap.description);
            println!();
        }
    }

    if !spec.personas.is_empty() {
        println!("## Personas");
        for persona in spec.personas.iter().sorted_by_key(|p| &p.name) {
            println!("### {}", persona.name);
            println!("{}", persona.description);
            println!();
        }
        println!();
    }

    println!("## Requirements");
    for requirement in spec.business_requirements.iter().sorted_by_key(|r| &r.id) {
        let markdown_level = requirement.markdown_level();
        println!(
            "{} {}. {}",
            "#".repeat(markdown_level),
            &requirement.id,
            &requirement.name
        );

        if let Some(date) = &requirement.date {
            let date = date.date.unwrap();
            println!("Date: {}-{}-{}\n", date.year, date.month, date.day)
        }

        if let Some(owner_id) = &requirement.owner_id {
            println!("Owner: {}\n", spec.find_contact(*owner_id));
        }

        if let Some(persona_id) = &requirement.persona_id {
            println!("Persona: {}\n", spec.find_persona(*persona_id).name);
        }

        if let Some(note) = &requirement.note {
            println!("{}", note);
        }
        println!();

        let technical_requirements = spec.find_technical_requirements_for(&requirement.id);
        if !technical_requirements.is_empty() {
            println!("{} Technical Requirements", "#".repeat(markdown_level + 1));

            for technical_requirement in technical_requirements {
                println!("{}\n", technical_requirement.description);

                if let Some(author_id) = &technical_requirement.author_id {
                    println!("Author: {}\n", spec.find_contact(*author_id));
                }

                if let Some(url) = &technical_requirement.code_url {
                    println!("[Code]({})\n", url);
                }

                if let Some(url) = &technical_requirement.test_url {
                    println!("[Tests]({})\n", url);
                }

                println!();
            }
        }
    }

    println!("## SLA");
    println!("{}", spec.sla);
    println!();

    if !spec.terms.is_empty() {
        println!("## Glossary");

        for term in spec.terms.iter().sorted_by_key(|t| &t.name) {
            println!("### {}", term.name);
            println!("{}", term.definition);
            println!();
        }
    }

    println!("## Contacts");
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
            println!("- {}", spec.find_contact(contact_id));
        }

        println!();
    }
}
