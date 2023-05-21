use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SpecMeta {
    pub version: u8,
    pub date: toml::value::Datetime,
    pub owner_id: u8,
    pub description: String,
    pub contacts: toml::Table,
}
