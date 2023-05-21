use serde::Deserialize;

use crate::business_requirement_id::BusinessRequirementId;

#[derive(Deserialize, Debug)]
pub struct TechnicalRequirement {
    pub requirement_id: BusinessRequirementId,
    pub author_id: Option<u8>,
    pub description: String,
    pub code_url: Option<String>,
    pub test_url: Option<String>,
}
