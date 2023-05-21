use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Term {
    pub name: String,
    pub definition: String,
}
