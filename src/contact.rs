use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Contact {
    pub id: u8,
    pub name: String,
    pub email: Option<String>,
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.email {
            Some(email) => write!(f, "[{}](mailto:{})", self.name, email),
            None => write!(f, "{}", self.name),
        }
    }
}
