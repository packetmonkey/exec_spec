use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Persona {
    pub id: u8,
    pub name: String,
    pub description: String,
}
