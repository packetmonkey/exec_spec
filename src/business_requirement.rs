use serde::Deserialize;

use crate::business_requirement_id::BusinessRequirementId;

#[derive(Deserialize, Clone, Debug)]
pub struct BusinessRequirement {
    pub id: BusinessRequirementId,
    pub name: String,
    pub date: Option<toml::value::Datetime>,
    pub note: Option<String>,
    pub owner_id: Option<u8>,
    pub persona_id: Option<u8>,
}

impl BusinessRequirement {
    pub fn markdown_level(&self) -> usize {
        self.id.elements_len() + 3
    }
}
