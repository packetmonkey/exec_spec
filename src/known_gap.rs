use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct KnownGap {
    pub requirement_id: String,
    pub description: String,
}
